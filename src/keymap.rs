use crate::action::{KeyAction, KeyboardAction};
use crate::event::{Debouncer, Event};
use crate::hid_report::HIDReport;
use crate::layers::Layers;
use heapless::Vec;

pub struct Chord {
    pub layer: usize,
    pub keys: (usize, usize),
    pub key_action: KeyAction,
}

pub struct Keymap<const L: usize, const N: usize, const DT: usize> {
    layers: Layers<L, N>,
    chords: &'static [Chord],
    events: [Event; N],
    debouncers: [Debouncer<DT>; N],
    hid_reports: Vec<HIDReport, 128>,
}

impl<const L: usize, const N: usize, const DT: usize> Keymap<L, N, DT> {
    pub fn new(layers: &'static [[KeyAction; N]; L], chords: &'static [Chord]) -> Keymap<L, N, DT> {
        Keymap {
            layers: Layers::<L, N>::new(layers),
            chords,
            events: [Event::Released(0); N],
            debouncers: [Debouncer::<DT>::new(); N],
            hid_reports: Vec::new(),
        }
    }

    pub fn update(&mut self, keys: &[bool]) {
        self.hid_reports.clear();

        for i in 0..N {
            self.events[i] = match keys[i] {
                true => self.debouncers[i].press(),
                false => self.debouncers[i].release(),
            };
        }

        self.chording();

        self.key_actions();
    }

    pub fn tick(&self) -> impl Iterator<Item = &HIDReport> + '_ {
        self.hid_reports.iter()
    }

    fn chording(&mut self) {
        for Chord {
            layer,
            keys: (key1, key2),
            key_action,
        } in self.chords
        {
            if *layer == self.layers.current_layer() {
                match (self.events[*key1], self.events[*key2]) {
                    (Event::Released(_), _)
                    | (_, Event::Released(_))
                    | (Event::Release(_), Event::Release(_)) => {}
                    (Event::Pressed(_), event)
                    | (event, Event::Pressed(_))
                    | (Event::Press(_), event)
                    | (event, Event::Press(_)) => {
                        self.events[*key1] = Event::Released(0);
                        self.events[*key2] = Event::Released(0);
                        self.key_action(
                            (key1 + key2) * (key1 + key2 + 1) / 2 + key2 + N,
                            key_action.event(&event),
                        )
                    }
                }
            }
        }
    }

    fn key_actions(&mut self) {
        for i in 0..N {
            self.key_action(i, self.layers.layer()[i].event(&self.events[i]));
        }
    }

    fn key_action(&mut self, id: usize, keyboard_action: Option<KeyboardAction>) {
        if let Some(keyboard_action) = keyboard_action {
            match keyboard_action {
                KeyboardAction::HIDReport(hid_report) => {
                    self.hid_reports.push(hid_report).ok();
                }
                KeyboardAction::LayerAction(layer_action) => {
                    self.layers.handle_layer_action(id, layer_action)
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::action::*;
    use crate::hid_report::*;
    use crate::holdtap::*;
    use crate::layers::*;
    use usbd_human_interface_device::page::*;

    static HT_1: KeyAction = KeyAction::CustomAction(&HoldTap {
        thold: 50,
        hold: kbc!(F),
        tap: kbc!(J),
    });

    static CHORDS: [Chord; 1] = [Chord {
        layer: 0,
        keys: (0, 2),
        key_action: kc!(Q),
    }];

    #[rustfmt::skip]
    static LAYERS: [[KeyAction; 6]; 2] = layers![
        [
            kc!(A), HT_1;
            kc!(A), kcl!(1);
            kc!(A), kdl!(1);
        ],
        [
            kc!(B), kc!(B);
            kc!(B), kul!(1);
            kc!(B), kdl!(0);
        ]
    ];

    fn bounce(keymap: &mut Keymap<2, 6, 5>, keys: &[bool]) {
        for _ in 0..5 {
            keymap.update(keys);
        }
    }

    #[test]
    fn test_keys() {
        let mut keymap = Keymap::<2, 6, 5>::new(&LAYERS, &CHORDS);

        #[rustfmt::skip]
        let mut keys = [false, false, false,
                                   false, false, false];

        for _ in 0..10 {
            keymap.update(&keys);
        }
        assert_eq!(keymap.tick().next(), None);

        // one key
        keys[0] = true;
        bounce(&mut keymap, &keys);
        for _ in 0..10 {
            keymap.update(&keys);
            assert_eq!(
                keymap.tick().next(),
                Some(&HIDReport::Keyboard(Keyboard::A))
            );
        }

        // two keys
        keys[1] = true;
        bounce(&mut keymap, &keys);
        for _ in 0..10 {
            keymap.update(&keys);
            let mut it = keymap.tick();
            assert_eq!(it.next(), Some(&HIDReport::Keyboard(Keyboard::A)));
            assert_eq!(it.next(), Some(&HIDReport::Keyboard(Keyboard::A)));
        }

        // clear
        keys.fill(false);
        bounce(&mut keymap, &keys);
        for _ in 0..10 {
            keymap.update(&keys);
            assert_eq!(keymap.tick().next(), None);
        }

        // layer 1
        keys[4] = true;
        keymap.update(&keys);
        keys[0] = true;
        bounce(&mut keymap, &keys);
        for _ in 0..10 {
            keymap.update(&keys);
            assert_eq!(
                keymap.tick().next(),
                Some(&HIDReport::Keyboard(Keyboard::B))
            );
        }

        // layer 0
        keys[4] = false;
        keymap.update(&keys);
        bounce(&mut keymap, &keys);
        for _ in 0..10 {
            keymap.update(&keys);
            assert_eq!(
                keymap.tick().next(),
                Some(&HIDReport::Keyboard(Keyboard::A))
            );
        }

        // layer 1
        keys[5] = true;
        keymap.update(&keys);
        bounce(&mut keymap, &keys);
        keys[5] = false;
        keymap.update(&keys);
        bounce(&mut keymap, &keys);
        for _ in 0..10 {
            keymap.update(&keys);
            assert_eq!(
                keymap.tick().next(),
                Some(&HIDReport::Keyboard(Keyboard::B))
            );
        }

        // layer 0
        keys[5] = true;
        keymap.update(&keys);
        bounce(&mut keymap, &keys);
        keys[5] = false;
        keymap.update(&keys);
        bounce(&mut keymap, &keys);
        for _ in 0..10 {
            keymap.update(&keys);
            assert_eq!(
                keymap.tick().next(),
                Some(&HIDReport::Keyboard(Keyboard::A))
            );
        }

        // layer 1
        keys[4] = true;
        keymap.update(&keys);
        bounce(&mut keymap, &keys);
        for _ in 0..10 {
            keymap.update(&keys);
            assert_eq!(
                keymap.tick().next(),
                Some(&HIDReport::Keyboard(Keyboard::B))
            );
        }

        // layer 0
        keys[5] = true;
        keymap.update(&keys);
        bounce(&mut keymap, &keys);
        for _ in 0..10 {
            keymap.update(&keys);
            assert_eq!(
                keymap.tick().next(),
                Some(&HIDReport::Keyboard(Keyboard::A))
            );
        }

        // layer 1
        keys[5] = false;
        keymap.update(&keys);
        bounce(&mut keymap, &keys);
        keys[5] = true;
        keymap.update(&keys);
        bounce(&mut keymap, &keys);
        for _ in 0..10 {
            keymap.update(&keys);
            assert_eq!(
                keymap.tick().next(),
                Some(&HIDReport::Keyboard(Keyboard::B))
            );
        }

        // layer 1
        keys[4] = true;
        keymap.update(&keys);
        bounce(&mut keymap, &keys);
        for _ in 0..10 {
            keymap.update(&keys);
            assert_eq!(
                keymap.tick().next(),
                Some(&HIDReport::Keyboard(Keyboard::B))
            );
        }

        // layer 1
        keys[4] = false;
        keymap.update(&keys);
        bounce(&mut keymap, &keys);
        for _ in 0..10 {
            keymap.update(&keys);
            assert_eq!(
                keymap.tick().next(),
                Some(&HIDReport::Keyboard(Keyboard::B))
            );
        }

        // layer 0
        keys[5] = false;
        keymap.update(&keys);
        bounce(&mut keymap, &keys);
        keys[5] = true;
        keymap.update(&keys);
        bounce(&mut keymap, &keys);
        for _ in 0..10 {
            keymap.update(&keys);
            assert_eq!(
                keymap.tick().next(),
                Some(&HIDReport::Keyboard(Keyboard::A))
            );
        }

        // clear
        keys.fill(false);
        keymap.update(&keys);
        bounce(&mut keymap, &keys);

        // hold
        keys[3] = true;
        bounce(&mut keymap, &keys);
        for _ in 0..51 {
            keymap.update(&keys);
        }
        assert_eq!(
            keymap.tick().next(),
            Some(&HIDReport::Keyboard(Keyboard::F))
        );

        // clear
        keys.fill(false);
        keymap.update(&keys);
        bounce(&mut keymap, &keys);

        // tap
        keys[3] = true;
        bounce(&mut keymap, &keys);
        for _ in 0..44 {
            keymap.update(&keys);
        }
        keys[3] = false;
        for _ in 0..6 {
            keymap.update(&keys);
        }
        assert_eq!(
            keymap.tick().next(),
            Some(&HIDReport::Keyboard(Keyboard::J))
        );

        // chording
        keys[0] = true;
        keymap.update(&keys);
        keys[2] = true;
        for i in 0..6 {
            keymap.update(&keys);
            if i == 6 {
                assert_eq!(
                    keymap.tick().next(),
                    Some(&HIDReport::Keyboard(Keyboard::A))
                );
            }
        }
        assert_eq!(
            keymap.tick().next(),
            Some(&HIDReport::Keyboard(Keyboard::Q))
        );
    }
}
