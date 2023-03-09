use heapless::spsc::Producer;

use crate::action::Action;
use crate::event::{Debouncer, Event};
use crate::handler::Handler;
use crate::performer::Performer;
use crate::report::Report;

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
        reports: Producer<'static, Report, 128>,
    ) -> Keymap<L, N, DT> {
        Keymap {
            layers,
            debouncers: [Debouncer::<DT>::new(); N],
            events: [Event::Released(0); N],
            handlers,
            performer: Performer::new(L, reports),
        }
    }

    pub fn tick(&mut self, keys: &[bool]) {
        for ((key, debouncer), event) in keys
            .iter()
            .zip(self.debouncers.iter_mut())
            .zip(self.events.iter_mut())
        {
            *event = match key {
                true => debouncer.press(),
                false => debouncer.release(),
            };
        }

        for handler in self.handlers {
            handler.handle(&mut self.events, &mut self.performer);
        }

        self.handle();
    }

    fn handle(&mut self) {
        for (i, (action, event)) in self.layers[self.performer.current_layer()]
            .iter()
            .zip(self.events.iter())
            .enumerate()
        {
            if let Some(keyboard_action) = action.act(event) {
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
    ([$([$($($x:expr),+ $(,)?);* $(;)?]),* $(,)?], $dt:literal, &$handlers:ident, $reports:ident) => {
        {
            const N_LAYERS: usize = count_layers!($([$($($x),*);*]),*);
            const N_KEYS: usize = count_keys!($([$($($x),*);*]),*);
            Keymap::<N_LAYERS, N_KEYS, $dt>::new(&layers!($([$($($x),*);*]),*), &$handlers, $reports)
        }
    };
}

#[cfg(test)]
mod test {
    use heapless::spsc::{Consumer, Queue};

    use crate::action::*;
    use crate::handler::*;
    use crate::keymap::Keymap;
    use crate::report::*;
    use crate::*;

    static mut Q: Queue<Report, 128> = Queue::new();
    static mut Q1: Queue<Report, 128> = Queue::new();

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

        let p = unsafe { Q.split().0 };
        keymap!([[
            kc!(A), ht!(50, F, J);
            kc!(A), prlc!(1);
            kc!(A), prld!(1);],
            [
            kc!(B), kc!(B);
            kc!(B), relu!(1);
            kc!(B), prld!(0);
            ]], 5, &HANDLERS, p);
    }

    fn bounce(
        keymap: &mut Keymap<2, 6, 5>,
        keys: &[bool; 6],
        c: &mut Consumer<'static, Report, 128>,
    ) {
        for _ in 0..5 {
            keymap.tick(keys);
        }
        while c.ready() {
            c.dequeue();
        }
    }

    #[test]
    fn test_keys() {
        let (p1, mut c1) = unsafe { Q1.split() };
        let mut keymap: Keymap<2, 6, 5> = keymap!([[
            kc!(A), ht!(50, F, J);
            kc!(A), prlc!(1);
            kc!(A), prld!(1);
        ],
        [
            kc!(B), kc!(B);
            kc!(B), relu!(1);
            kc!(B), prld!(0);
        ]], 5, &HANDLERS, p1);

        let mut keys = [false, false, false, false, false, false];

        for _ in 0..10 {
            keymap.tick(&keys);
        }
        assert_eq!(c1.dequeue(), None);

        // one key
        keys[0] = true;
        bounce(&mut keymap, &keys, &mut c1);
        for _ in 0..10 {
            keymap.tick(&keys);
            assert_eq!(c1.dequeue(), Some(Report::Keyboard(Keyboard::A)));
        }

        // two keys
        keys[1] = true;
        bounce(&mut keymap, &keys, &mut c1);
        for _ in 0..10 {
            keymap.tick(&keys);
            assert_eq!(c1.dequeue(), Some(Report::Keyboard(Keyboard::A)));
            assert_eq!(c1.dequeue(), Some(Report::Keyboard(Keyboard::A)));
        }

        // clear
        keys.fill(false);
        bounce(&mut keymap, &keys, &mut c1);
        for _ in 0..10 {
            keymap.tick(&keys);
            assert_eq!(c1.dequeue(), None);
        }

        // layer 1
        keys[4] = true;
        keymap.tick(&keys);
        keys[0] = true;
        bounce(&mut keymap, &keys, &mut c1);
        for _ in 0..10 {
            keymap.tick(&keys);
            assert_eq!(c1.dequeue(), Some(Report::Keyboard(Keyboard::B)));
        }

        // layer 0
        keys[4] = false;
        keymap.tick(&keys);
        bounce(&mut keymap, &keys, &mut c1);
        for _ in 0..10 {
            keymap.tick(&keys);
            assert_eq!(c1.dequeue(), Some(Report::Keyboard(Keyboard::A)));
        }

        // layer 1
        keys[5] = true;
        keymap.tick(&keys);
        bounce(&mut keymap, &keys, &mut c1);
        keys[5] = false;
        keymap.tick(&keys);
        bounce(&mut keymap, &keys, &mut c1);
        for _ in 0..10 {
            keymap.tick(&keys);
            assert_eq!(c1.dequeue(), Some(Report::Keyboard(Keyboard::B)));
        }

        // layer 0
        keys[5] = true;
        keymap.tick(&keys);
        bounce(&mut keymap, &keys, &mut c1);
        keys[5] = false;
        keymap.tick(&keys);
        bounce(&mut keymap, &keys, &mut c1);
        for _ in 0..10 {
            keymap.tick(&keys);
            assert_eq!(c1.dequeue(), Some(Report::Keyboard(Keyboard::A)));
        }

        // layer 1
        keys[4] = true;
        keymap.tick(&keys);
        bounce(&mut keymap, &keys, &mut c1);
        for _ in 0..10 {
            keymap.tick(&keys);
            assert_eq!(c1.dequeue(), Some(Report::Keyboard(Keyboard::B)));
        }

        // layer 0
        keys[5] = true;
        keymap.tick(&keys);
        bounce(&mut keymap, &keys, &mut c1);
        for _ in 0..10 {
            keymap.tick(&keys);
            assert_eq!(c1.dequeue(), Some(Report::Keyboard(Keyboard::A)));
        }

        // layer 1
        keys[5] = false;
        keymap.tick(&keys);
        bounce(&mut keymap, &keys, &mut c1);
        keys[5] = true;
        keymap.tick(&keys);
        bounce(&mut keymap, &keys, &mut c1);
        for _ in 0..10 {
            keymap.tick(&keys);
            assert_eq!(c1.dequeue(), Some(Report::Keyboard(Keyboard::B)));
        }

        // layer 1
        keys[4] = true;
        keymap.tick(&keys);
        bounce(&mut keymap, &keys, &mut c1);
        for _ in 0..10 {
            keymap.tick(&keys);
            assert_eq!(c1.dequeue(), Some(Report::Keyboard(Keyboard::B)));
        }

        // layer 1
        keys[4] = false;
        keymap.tick(&keys);
        bounce(&mut keymap, &keys, &mut c1);
        for _ in 0..10 {
            keymap.tick(&keys);
            assert_eq!(c1.dequeue(), Some(Report::Keyboard(Keyboard::B)));
        }

        // layer 0
        keys[5] = false;
        keymap.tick(&keys);
        bounce(&mut keymap, &keys, &mut c1);
        keys[5] = true;
        keymap.tick(&keys);
        bounce(&mut keymap, &keys, &mut c1);
        for _ in 0..10 {
            keymap.tick(&keys);
            assert_eq!(c1.dequeue(), Some(Report::Keyboard(Keyboard::A)));
        }

        // clear
        keys.fill(false);
        keymap.tick(&keys);
        bounce(&mut keymap, &keys, &mut c1);

        // hold
        keys[3] = true;
        bounce(&mut keymap, &keys, &mut c1);
        for _ in 0..51 {
            keymap.tick(&keys);
        }
        assert_eq!(c1.dequeue(), Some(Report::Keyboard(Keyboard::F)));

        // clear
        keys.fill(false);
        keymap.tick(&keys);
        bounce(&mut keymap, &keys, &mut c1);

        // tap
        keys[3] = true;
        bounce(&mut keymap, &keys, &mut c1);
        for _ in 0..44 {
            keymap.tick(&keys);
        }
        keys[3] = false;
        for _ in 0..6 {
            keymap.tick(&keys);
        }
        assert_eq!(c1.dequeue(), Some(Report::Keyboard(Keyboard::J)));

        // chording
        keys[2] = true;
        keymap.tick(&keys);
        keys[0] = true;
        for _ in 0..6 {
            keymap.tick(&keys);
        }
        assert_eq!(c1.dequeue(), Some(Report::Keyboard(Keyboard::Q)));
        keys[0] = false;
        keys[2] = false;
        bounce(&mut keymap, &keys, &mut c1);

        // chording
        keys[5] = true;
        keymap.tick(&keys);
        keys[2] = true;
        for _ in 0..6 {
            keymap.tick(&keys);
        }

        assert_eq!(c1.dequeue(), Some(Report::Keyboard(Keyboard::C)));
        assert_eq!(c1.dequeue(), Some(Report::Keyboard(Keyboard::D)));
    }
}
