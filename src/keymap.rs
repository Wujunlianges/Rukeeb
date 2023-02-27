use crate::action::Action;
use crate::event::{Debouncer, Event};
use crate::handler::Handler;
use crate::hid_report::HIDReport;
use crate::register::Register;

pub struct Keymap<const L: usize, const N: usize, const DT: usize> {
    layers: &'static [[Action; N]; L],
    debouncers: [Debouncer<DT>; N],
    events: [Event; N],
    handlers: &'static [Handler<L, N>],
    register: Register<L, N>,
}

impl<const L: usize, const N: usize, const DT: usize> Keymap<L, N, DT> {
    pub fn new(
        layers: &'static [[Action; N]; L],
        handlers: &'static [Handler<L, N>],
    ) -> Keymap<L, N, DT> {
        Keymap {
            layers,
            debouncers: [Debouncer::<DT>::new(); N],
            events: [Event::Released(0); N],
            handlers,
            register: Register::<L, N>::new(),
        }
    }

    pub fn update(&mut self, keys: &[bool]) {
        self.register.clear();

        for i in 0..N {
            self.events[i] = match keys[i] {
                true => self.debouncers[i].press(),
                false => self.debouncers[i].release(),
            };
        }

        for handler in self.handlers {
            handler.handle(&mut self.events, &mut self.register);
        }

        self.key_actions();
    }

    pub fn tick(&self) -> impl Iterator<Item = &HIDReport> + '_ {
        self.register.tick()
    }

    fn key_actions(&mut self) {
        for i in 0..N {
            if let Some(keyboard_action) =
                self.layers[self.register.current_layer()][i].event(&self.events[i])
            {
                self.register.register(i, &keyboard_action);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::action::*;
    use crate::handler::*;
    use crate::hid_report::*;
    use crate::holdtap::*;
    use crate::*;
    use usbd_human_interface_device::page::Keyboard;

    static CHORD1: Chord<2> = Chord {
        keys: (0, 2),
        keyboard_actions: [Some(k!(Q)), None],
    };

    static COMB1_KEYS: [KeyboardAction; 2] = [k!(C), k!(D)];
    static COMB1: Comb<2> = Comb {
        key: 2,
        keyboard_actions: [None, Some(&COMB1_KEYS)],
    };
    static HANDLERS: [Handler<2, 6>; 2] = [Handler(&CHORD1), Handler(&COMB1)];

    #[rustfmt::skip]
    static LAYERS: [[Action; 6]; 2] = layers![
        [
            pdk!(A), ht!(50, F, J);
            pdk!(A), plc!(1);
            pdk!(A), pld!(1);
        ],
        [
            pdk!(B), pdk!(B);
            pdk!(B), rlu!(1);
            pdk!(B), pld!(0);
        ]
    ];

    fn bounce(keymap: &mut Keymap<2, 6, 5>, keys: &[bool]) {
        for _ in 0..5 {
            keymap.update(keys);
        }
    }

    #[test]
    fn test_keys() {
        let mut keymap = Keymap::<2, 6, 5>::new(&LAYERS, &HANDLERS);

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
        keys[2] = true;
        keymap.update(&keys);
        keys[0] = true;
        for _ in 0..6 {
            keymap.update(&keys);
        }
        assert_eq!(
            keymap.tick().next(),
            Some(&HIDReport::Keyboard(Keyboard::Q))
        );
        keys[0] = false;
        keys[2] = false;
        bounce(&mut keymap, &keys);

        // chording
        keys[5] = true;
        keymap.update(&keys);
        keys[2] = true;
        for _ in 0..6 {
            keymap.update(&keys);
        }
        let mut it = keymap.tick();
        assert_eq!(it.next(), Some(&HIDReport::Keyboard(Keyboard::C)));
        assert_eq!(it.next(), Some(&HIDReport::Keyboard(Keyboard::D)));
    }
}
