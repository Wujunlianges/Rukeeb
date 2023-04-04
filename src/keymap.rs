use heapless::spsc::Producer;

use crate::action::Act;
use crate::event::{Debouncer, Event};
use crate::handler::Handle;
use crate::performer::Performer;
use crate::report::Report;

pub struct Keymap<const L: usize, const N: usize, const DT: usize> {
    layers: &'static [[&'static dyn Act; N]; L],
    debouncers: [Debouncer<DT>; N],
    events: [Event; N],
    handlers: &'static [&'static dyn Handle],
    performer: Performer,
}

impl<const L: usize, const N: usize, const DT: usize> Keymap<L, N, DT> {
    pub fn new(
        layers: &'static [[&'static dyn Act; N]; L],
        handlers: &'static [&'static dyn Handle],
        reports: Producer<'static, Report, 128>,
    ) -> Keymap<L, N, DT> {
        Keymap {
            layers,
            debouncers: [Debouncer::new(); N],
            events: [Event::new(); N],
            handlers,
            performer: Performer::new(L, reports),
        }
    }

    pub fn tick(&mut self, switches: &[bool]) {
        for ((switch, debouncer), event) in switches
            .iter()
            .zip(self.debouncers.iter_mut())
            .zip(self.events.iter_mut())
        {
            *event = match switch {
                true => debouncer.press(),
                false => debouncer.release(),
            };
        }

        for (i, handler) in self.handlers.iter().enumerate() {
            handler.handle(N + i, &mut self.events, &mut self.performer);
        }

        self.handle();
    }

    fn handle(&mut self) {
        for (i, (key, event)) in self.layers[self.performer.current_layer()]
            .iter()
            .zip(self.events.iter())
            .enumerate()
        {
            if let Some(action) = key.act(event) {
                self.performer.perform(i, action);
            }
        }
    }
}

// From https://veykril.github.io/tlborm/decl-macros/building-blocks/counting.html#bit-twiddling
macro_rules! count_tts {
    () => { 0 };
    ($odd:tt $($a:tt $b:tt)*) => { ($crate::keymap::count_tts!($($a)*) << 1) | 1 };
    ($($a:tt $even:tt)*) => { $crate::keymap::count_tts!($($a)*) << 1 };
}
pub(crate) use count_tts;

macro_rules! count_layers {
    ($([$($($x:expr),*);*]),*) => {$crate::keymap::count_tts!($([$($($x),*);*])*)};
}
pub(crate) use count_layers;

macro_rules! count_keys {
    ([$($($x0:expr),*);*], $([$($($x:expr),*);*]),*) => {$crate::keymap::count_tts!($($($x0)*)*)};
}
pub(crate) use count_keys;

macro_rules! layer {
    ($($($x: expr,)*;)*) => {
        $crate::keymap::layer!(@flatten [] $($($x,)*;)*)
    };
    (@flatten [$($col:expr,)*] $($x0:expr, $($x:expr,)*;)*) => {
        $crate::keymap::layer!(@flatten [$($col,)* $($x0,)*] $($($x,)*;)*)
    };
    (@flatten [$($col:expr,)*] $(;)*) => {
        [$($col,)*]
    };
}
pub(crate) use layer;

#[macro_export]
macro_rules! layers {
    ($([$($($x:expr),+ $(,)?);* $(;)?]),* $(,)?) => {
        [$($crate::keymap::layer!($($($x,)*;)*),)*]
    };
}

#[macro_export]
macro_rules! keymap {
    ([$([$($($x:expr),+ $(,)?);* $(;)?]),* $(,)?], $dt:literal, &$handlers:ident, $reports:ident) => {
        {
            const N_LAYERS: usize = $crate::keymap::count_layers!($([$($($x),*);*]),*);
            const N_KEYS: usize = $crate::keymap::count_keys!($([$($($x),*);*]),*);
            const LAYERS: [[&'static dyn $crate::action::Act; N_KEYS] ;N_LAYERS] = $crate::layers!($([$($(&$x),*);*]),*);
            Keymap::<N_LAYERS, N_KEYS, $dt>::new(&LAYERS, &$handlers, $reports)
        }
    };
}

#[cfg(test)]
#[no_implicit_prelude]
mod test_macros {
    extern crate core;
    extern crate heapless;
    extern crate std;

    macro_rules! layer {
        ($($($x: expr,)*;)*) => {
            "Wrong macro!"
        };
    }
    macro_rules! count_tts {
        () => {
            "Wrong macro!"
        };
        ($odd:tt $($a:tt $b:tt)*) => {
            "Wrong macro!"
        };
        ($($a:tt $even:tt)*) => {
            "Wrong macro!"
        };
    }

    use core::sync::atomic::AtomicBool;
    use heapless::spsc::Queue;
    use std::option::Option::{None, Some};

    use crate::action::Action;
    use crate::handler::{Chord, Comb, Handle};
    use crate::keymap::Keymap;
    use crate::report::Report;
    use crate::*;

    static mut Q: Queue<Report, 128> = Queue::new();

    static CHORD1: Chord<2> = chrd!(0, 2, [Some(kb!(Q)), None]);
    static COMB1: Comb<2> = comb!(2, [None, Some(&[kb!(C), kb!(D)])]);
    static HANDLERS: [&'static dyn Handle; 2] = handlers![CHORD1, COMB1];

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
            kc!(A), ht!(50, kb!(F), kb!(J));
            kc!(A), prlc!(1);
            kc!(A), prld!(1);],
            [
            kc!(B), kc!(B);
            kc!(B), relu!(1);
            kc!(B), prld!(0);
            ]], 5, &HANDLERS, p);
    }
}

#[cfg(test)]
mod test {
    use core::sync::atomic::AtomicBool;

    use heapless::spsc::{Consumer, Queue};

    use crate::action::*;
    use crate::handler::*;
    use crate::keymap::Keymap;
    use crate::report::*;
    use crate::*;

    static mut Q: Queue<Report, 128> = Queue::new();

    static CHORD1: Chord<2> = chrd!(0, 2, [Some(kb!(Q)), None]);
    static COMB1: Comb<2> = comb!(2, [None, Some(&[kb!(C), kb!(D)])]);
    static HANDLERS: [&'static dyn Handle; 2] = handlers![CHORD1, COMB1];

    macro_rules! r {
        ($x:tt) => {
            Report::Keyboard(Keyboard::$x)
        };
    }

    struct Tester<const L: usize, const N: usize, const DT: usize> {
        keymap: Keymap<L, N, DT>,
        c: Consumer<'static, Report, 128>,
    }

    impl<const L: usize, const N: usize, const DT: usize> Tester<L, N, DT> {
        pub fn new(
            keymap: Keymap<L, N, DT>,
            c: Consumer<'static, Report, 128>,
        ) -> Tester<L, N, DT> {
            Tester { keymap, c }
        }

        fn reset(&mut self) {
            for _ in 0..(DT + 1) {
                self.keymap.tick(&[false; N]);
            }
            while self.c.ready() {
                self.c.dequeue();
            }
        }

        pub fn test(&mut self, ids: &[usize], delays: &[usize], expected_outputs: &[Report]) {
            let mut switches = [false; N];

            assert!(!ids.is_empty());
            assert!(delays.len() == ids.len());
            for id in ids {
                assert!(*id < N);
            }

            self.reset();
            for (id, delay) in ids.iter().zip(delays[0..ids.len()].iter()) {
                switches[*id] ^= true;
                for _ in 0..*delay {
                    self.keymap.tick(&switches)
                }
                while self.c.ready() {
                    self.c.dequeue();
                }
            }
            self.keymap.tick(&switches);
            for expected_output in expected_outputs {
                assert_eq!(self.c.dequeue().unwrap(), *expected_output);
            }
        }
    }

    #[test]
    fn test() {
        let (p, c) = unsafe { Q.split() };
        let keymap: Keymap<2, 6, 5> = keymap!([[
            kc!(A), ht!(50, kb!(F), kb!(J));
            kc!(A), prlc!(1);
            kc!(A), prld!(1);
        ],
        [
            kc!(B), kc!(B);
            kc!(B), relu!(1);
            kc!(B), prld!(0);
        ]], 5, &HANDLERS, p);

        let mut tester = Tester::new(keymap, c);

        // 1 key
        tester.test(&[0], &[5], &[r!(A)]);
        // 2 keys
        tester.test(&[0, 1], &[0, 5], &[r!(A), r!(A)]);
        // current layer 0 -> 1
        tester.test(&[4, 0], &[6, 5], &[r!(B)]);
        // current layer 1 -> 0 -> 1 -> 0
        tester.test(&[4, 4, 0], &[6, 6, 5], &[r!(A)]);
        // default layer 0 -> 1
        tester.test(&[5, 5, 0], &[6, 0, 5], &[r!(B)]);
        // default layer 1 -> 0
        tester.test(&[5, 5, 0], &[6, 6, 5], &[r!(A)]);
        // current layer 0 -> 1
        // default layer 1 -> 0
        tester.test(&[4, 5, 0], &[6, 6, 5], &[r!(A)]);
        // default layer 0 -> 1
        // // current layer 1 -> 0
        tester.test(&[5, 4, 0], &[6, 6, 5], &[r!(B)]);
        // default layer 1 -> 0
        tester.test(&[5, 0], &[6, 5], &[r!(A)]);
        // tap
        tester.test(&[3, 3], &[49, 5], &[r!(J)]);
        // hold
        tester.test(&[3], &[55], &[r!(F)]);
        //chording
        tester.test(&[0, 2], &[0, 5], &[r!(Q)]);
        // combination
        tester.test(&[4, 2], &[6, 5], &[r!(C), r!(D)]);
    }
}
