use heapless::spsc::Producer;

use crate::action::Act;
use crate::debouncer::Debouncer;
use crate::event::Event;
use crate::handler::{Handle, KeyHandler};
use crate::performer::Performer;
use crate::report::Report;
use crate::state::State;

pub struct Keymap<const N: usize, const L: usize, const DT: usize> {
    debouncers: [Debouncer<DT>; N],
    states: [State; N],
    handlers: &'static [&'static dyn Handle<N, L>],
    key_handler: KeyHandler<N, L>,
    performer: Performer<L>,
}

impl<const N: usize, const L: usize, const DT: usize> Keymap<N, L, DT> {
    pub fn new(
        keys: &'static [[&'static dyn Act; L]; N],
        handlers: &'static [&'static dyn Handle<N, L>],
        reports: Producer<'static, Report, 128>,
    ) -> Keymap<N, L, DT> {
        Keymap {
            debouncers: [Debouncer::new(); N],
            states: [State::new(); N],
            handlers,
            key_handler: KeyHandler::new(keys),
            performer: Performer::new(reports),
        }
    }

    pub fn tick(&mut self, switches: &[bool]) {
        switches
            .iter()
            .zip(self.debouncers.iter_mut())
            .zip(self.states.iter_mut())
            .for_each(|((switch, debouncer), state)| {
                state.update(debouncer.trigger(*switch), self.performer.current_layer())
            });

        self.handlers
            .iter()
            .for_each(|handler| handler.handle(&mut self.states, &mut self.performer));

        self.key_handler
            .handle(&mut self.states, &mut self.performer);
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

#[macro_export]
macro_rules! keymap {
    ([$([$($($x:expr),+ $(,)?);* $(;)?]),* $(,)?], $dt:literal, &$handlers:ident, $reports:ident) => {
        {
            const N_LAYERS: usize = $crate::keymap::count_layers!($([$($($x),*);*]),*);
            const N_KEYS: usize = $crate::keymap::count_keys!($([$($($x),*);*]),*);
            const KEYS: [[&'static dyn $crate::action::Act; N_LAYERS] ;N_KEYS] = $crate::keys!($([$($($x),*);*]),*);
            $crate::keymap::Keymap::<N_KEYS, N_LAYERS, $dt>::new(&KEYS, &$handlers, $reports)
        }
    };
}

#[cfg(test)]
mod test {
    use heapless::spsc::{Consumer, Queue};

    use crate::handler::chord::Chord;
    use crate::handler::comb::Comb;
    use crate::handler::*;
    use crate::keymap::Keymap;
    use crate::report::*;
    use crate::*;

    static mut Q: Queue<Report, 128> = Queue::new();

    static CHORD1: Chord<2> = chrd!(0, 2, [Some(kb!(Q)), None]);
    static COMB1: Comb<2> = cmb!(2, [None, Some(&[kb!(C), kb!(D)])]);
    static HANDLERS: [&'static dyn Handle<6, 2>; 2] = [&CHORD1, &COMB1];

    macro_rules! r {
        ($x:tt) => {
            Report::Keyboard(Keyboard::$x)
        };
    }

    struct Tester<const N: usize, const L: usize, const DT: usize> {
        keymap: Keymap<N, L, DT>,
        c: Consumer<'static, Report, 128>,
    }

    impl<const N: usize, const L: usize, const DT: usize> Tester<N, L, DT> {
        pub fn new(
            keymap: Keymap<N, L, DT>,
            c: Consumer<'static, Report, 128>,
        ) -> Tester<N, L, DT> {
            Tester { keymap, c }
        }

        fn reset(&mut self) {
            (0..(DT + 1)).into_iter().for_each(|_| {
                self.keymap.tick(&[false; N]);
            });
            while self.c.ready() {
                self.c.dequeue();
            }
        }

        pub fn test(&mut self, ids: &[usize], delays: &[usize], expected_outputs: &[Report]) {
            let mut switches = [false; N];

            assert!(!ids.is_empty());
            assert!(delays.len() == ids.len());
            ids.iter().for_each(|id| {
                assert!(*id < N);
            });

            self.reset();
            ids.iter().zip(delays.iter()).for_each(|(id, delay)| {
                switches[*id] ^= true;
                (0..*delay).for_each(|_| {
                    self.keymap.tick(&switches);
                });
                while self.c.ready() {
                    self.c.dequeue();
                }
            });

            self.keymap.tick(&switches);
            expected_outputs.iter().for_each(|expected_output| {
                assert_eq!(
                    self.c.dequeue().unwrap(),
                    *expected_output,
                    "Inputs: {:?} {:?} {:?}",
                    ids,
                    delays,
                    expected_outputs
                );
            });
        }
    }

    #[test]
    fn test() {
        let (p, c) = unsafe { Q.split() };
        let keymap: Keymap<6, 2, 5> = keymap!([[
            kc!(A), ht!(50, kb!(F), kb!(J));
            kc!(A), oocl!(1);
            kc!(A), tpdl!(1);
        ],
        [
            kc!(B), kc!(B);
            kc!(B), kc!(B);
            kc!(B), tpdl!(0);
        ]], 5, &HANDLERS, p);

        let mut tester = Tester::new(keymap, c);
        // 1 key
        tester.test(&[0], &[5], &[r!(A)]);
        // 2 keys
        tester.test(&[0, 1], &[0, 5], &[r!(A), r!(A)]);
        // current layer 0 -> 1 -> 0
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
        // chording
        tester.test(&[0, 2], &[0, 5], &[r!(Q)]);
        // combination
        tester.test(&[4, 2], &[6, 5], &[r!(C), r!(D)]);
    }
}
