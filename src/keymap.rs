use crate::action::Action;
use crate::event::{Debouncer, Event};
use crate::handler::Handler;
use crate::report::Report;
use crate::performer::Performer;

pub struct Keymap<const L: usize, const N: usize, const DT: usize> {
    layers: &'static [[Action; N]; L],
    debouncers: [Debouncer<DT>; N],
    events: [Event; N],
    handlers: &'static [Handler],
    performer: Performer,
}

impl<const L: usize, const N: usize, const DT: usize> Keymap<L, N, DT> {
    pub fn new(
        layers: &'static [[Action; N]; L],
        handlers: &'static [Handler],
    ) -> Keymap<L, N, DT> {
        Keymap {
            layers,
            debouncers: [Debouncer::<DT>::new(); N],
            events: [Event::Released(0); N],
            handlers,
            performer: Performer::new(L),
        }
    }

    pub fn update(&mut self, keys: &[bool]) {
        self.performer.clear();

        for i in 0..N {
            self.events[i] = match keys[i] {
                true => self.debouncers[i].press(),
                false => self.debouncers[i].release(),
            };
        }

        for handler in self.handlers {
            handler.handle(&mut self.events, &mut self.performer);
        }

        self.key_actions();
    }

    pub fn tick(&self) -> impl Iterator<Item = &Report> + '_ {
        self.performer.tick()
    }

    fn key_actions(&mut self) {
        for i in 0..N {
            if let Some(keyboard_action) =
                self.layers[self.performer.current_layer()][i].event(&self.events[i])
            {
                self.performer.perform(i, &keyboard_action);
            }
        }
    }
}

// From https://veykril.github.io/tlborm/decl-macros/building-blocks/counting.html#bit-twiddling
macro_rules! count_tts {
    () => { 0 };
    ($odd:tt $($a:tt $b:tt)*) => { (count_tts!($($a)*) << 1) | 1 };
    ($($a:tt $even:tt)*) => { count_tts!($($a)*) << 1 };
}

macro_rules! count_layers {
    ($([$($($x:expr),*);*]),*) => {count_tts!($([$($($x),*);*])*)};
}

macro_rules! count_keys {
    ([$($($x0:expr),*);*], $([$($($x:expr),*);*]),*) => {count_tts!($($($x0)*)*)};
}

macro_rules! layer {
    ($($($x: expr,)*;)*) => {
        layer!(@flatten [] $($($x,)*;)*)
    };
    (@flatten [$($col:expr,)*] $($x0:expr, $($x:expr,)*;)*) => {
        layer!(@flatten [$($col,)* $($x0,)*] $($($x,)*;)*)
    };
    (@flatten [$($col:expr,)*] $(;)*) => {
        [$($col,)*]
    };
}

#[macro_export]
macro_rules! layers {
    ($([$($($x:expr),+ $(,)?);* $(;)?]),* $(,)?) => {
        [$(layer!($($($x,)*;)*),)*]
    };
}

#[macro_export]
macro_rules! keymap {
    ([$([$($($x:expr),+ $(,)?);* $(;)?]),* $(,)?], &$handlers:ident, $dt:literal) => {
        {
            const N_LAYERS: usize = count_layers!($([$($($x),*);*]),*);
            const N_KEYS: usize = count_keys!($([$($($x),*);*]),*);
            Keymap::<N_LAYERS, N_KEYS, $dt>::new(&layers!($([$($($x),*);*]),*), &$handlers)
        }
    };
}

#[cfg(test)]
mod test {
    use crate::action::*;
    use crate::handler::*;
    use crate::report::*;
    use crate::keymap::Keymap;
    use crate::*;

    static CHORD1: Chord<2> = Chord {
        keys: (0, 2),
        keyboard_actions: [Some(kb!(Q)), None],
    };

    static COMB1_KEYS: [KeyboardAction; 2] = [kb!(C), kb!(D)];
    static COMB1: Comb<2> = Comb {
        key: 2,
        keyboard_actions: [None, Some(&COMB1_KEYS)],
    };
    static HANDLERS: [Handler; 2] = [Handler(&CHORD1), Handler(&COMB1)];

    #[test]
    fn test_layer_macros() {
        layers![[kc!(A)]];
        layers![[kc!(A),]];
        layers![[kc!(A);]];
        layers![[kc!(A),;]];

        layers![[kc!(A),;kc!(A)]];
        layers![[kc!(A),;kc!(A);]];

        layers![[kc!(A),kc!(A);kc!(A),kc!(A),;]];

        keymap!([[
            kc!(A), ht!(50, F, J);
            kc!(A), prlc!(1);
            kc!(A), prld!(1);],
            [
            kc!(B), kc!(B);
            kc!(B), relu!(1);
            kc!(B), prld!(0);
            ]], &HANDLERS, 5);
    }

    fn bounce(keymap: &mut Keymap<2, 6, 5>, keys: &[bool]) {
        for _ in 0..5 {
            keymap.update(keys);
        }
    }

    #[test]
    fn test_keys() {
        let mut keymap: Keymap<2, 6, 5> = keymap!([[
            kc!(A), ht!(50, F, J);
            kc!(A), prlc!(1);
            kc!(A), prld!(1);
        ],
        [
            kc!(B), kc!(B);
            kc!(B), relu!(1);
            kc!(B), prld!(0);
        ]], &HANDLERS, 5);

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
                Some(&Report::Keyboard(Keyboard::A))
            );
        }

        // two keys
        keys[1] = true;
        bounce(&mut keymap, &keys);
        for _ in 0..10 {
            keymap.update(&keys);
            let mut it = keymap.tick();
            assert_eq!(it.next(), Some(&Report::Keyboard(Keyboard::A)));
            assert_eq!(it.next(), Some(&Report::Keyboard(Keyboard::A)));
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
                Some(&Report::Keyboard(Keyboard::B))
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
                Some(&Report::Keyboard(Keyboard::A))
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
                Some(&Report::Keyboard(Keyboard::B))
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
                Some(&Report::Keyboard(Keyboard::A))
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
                Some(&Report::Keyboard(Keyboard::B))
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
                Some(&Report::Keyboard(Keyboard::A))
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
                Some(&Report::Keyboard(Keyboard::B))
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
                Some(&Report::Keyboard(Keyboard::B))
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
                Some(&Report::Keyboard(Keyboard::B))
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
                Some(&Report::Keyboard(Keyboard::A))
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
            Some(&Report::Keyboard(Keyboard::F))
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
            Some(&Report::Keyboard(Keyboard::J))
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
            Some(&Report::Keyboard(Keyboard::Q))
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
        assert_eq!(it.next(), Some(&Report::Keyboard(Keyboard::C)));
        assert_eq!(it.next(), Some(&Report::Keyboard(Keyboard::D)));
    }
}
