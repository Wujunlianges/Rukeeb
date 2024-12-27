use heapless::spsc::Producer;
use itertools::izip;

use crate::debouncer::{Debounce, Debouncer};
use crate::event::Event;
use crate::function::Function;
use crate::handler::Handle;
use crate::processor::Process;
use crate::report::Report;

const MAX_REPORTS: usize = 128;
const DT: usize = 5;

trait Keymap<const N: usize, const L: usize> {
    type DB: Debounce;
    fn tick(&mut self, switches: &[bool; N]);
}

pub struct BasicKeymap<const N: usize, const L: usize> {
    events: [Event; N],
    layer: usize,
    debouncers: [Debouncer<DT>; N],
    handlers: [Option<&'static dyn Handle>; N],
    processors: &'static [&'static dyn Process<N, L>],
    reporter: Producer<'static, Report, MAX_REPORTS>,
}

impl<const N: usize, const L: usize> Keymap<N, L> for BasicKeymap<N, L> {
    type DB = Debouncer<DT>;

    fn tick(&mut self, switches: &[bool; N]) {
        for (event, debouncer, switch) in izip!(&mut self.events, &mut self.debouncers, switches) {
            *event = debouncer.debounce(*switch);
        }

        // Process all events.
        self.processors
            .iter()
            .for_each(|handler| handler.process(&mut self.handlers, &self.events, self.layer));

        // Handle individual events.
        for (handler, event) in izip!(&mut self.handlers, &self.events) {
            if let Some(handler) = handler {
                if let Some(function) = handler.handle(event) {
                    match function {
                        Function::Report(report) => self.reporter.enqueue(*report).unwrap(),
                        Function::Layer(layer) => self.layer = *layer,
                    }
                }
            }
            if matches!(event, Event::Released(_)) {
                *handler = None;
            }
        }
    }
}

impl<const N: usize, const L: usize> BasicKeymap<N, L> {
    pub fn new(
        processors: &'static [&'static dyn Process<N, L>],
        reporter: Producer<'static, Report, MAX_REPORTS>,
    ) -> BasicKeymap<N, L> {
        BasicKeymap {
            events: [Event::default(); N],
            handlers: [None; N],
            layer: 0,
            debouncers: [Debouncer::<DT>::new(); N],
            processors,
            reporter,
        }
    }
}

#[cfg(test)]
mod test {
    use heapless::spsc::{Consumer, Queue};

    use crate::debouncer::{Debounce, Debouncer};
    use crate::handler::Handle;
    use crate::keymap::{BasicKeymap, Keymap};
    use crate::processor::chord::Chord;
    use crate::processor::{KeyProcessor, Process};
    use crate::report::{Keyboard, Report};
    use crate::*;

    const MAX_REPORTS: usize = 128;

    macro_rules! r {
        ($x:tt) => {
            Report::Keyboard(Keyboard::$x)
        };
    }

    struct Tester<const N: usize, const L: usize> {
        keymap: BasicKeymap<N, L>,
        consumer: Consumer<'static, Report, MAX_REPORTS>,
    }

    impl<const N: usize, const L: usize> Tester<N, L> {
        pub fn new(
            keymap: BasicKeymap<N, L>,
            consumer: Consumer<'static, Report, MAX_REPORTS>,
        ) -> Tester<N, L> {
            Tester { keymap, consumer }
        }

        fn reset_keys(&mut self) {
            (0..11).for_each(|_| {
                self.keymap.tick(&[false; N]);
            });
            while self.consumer.ready() {
                self.consumer.dequeue();
            }
        }

        pub fn test(&mut self, ids: &[usize], delays: &[usize], expected_outputs: &[Report]) {
            let mut switches = [false; N];

            assert!(!ids.is_empty());
            assert!(delays.len() == ids.len());
            ids.iter().for_each(|id| {
                assert!(*id < N);
            });

            self.reset_keys();
            ids.iter().zip(delays.iter()).for_each(|(id, delay)| {
                switches[*id] ^= true;
                (0..*delay).for_each(|_| {
                    self.keymap.tick(&switches);
                });
                while self.consumer.ready() {
                    self.consumer.dequeue();
                }
            });

            self.keymap.tick(&switches);
            expected_outputs.iter().for_each(|expected_output| {
                assert_eq!(
                    self.consumer.dequeue().unwrap(),
                    *expected_output,
                    "Inputs: {:?} {:?} {:?}",
                    ids,
                    delays,
                    expected_outputs
                );
            });
        }
    }

    static mut Q: Queue<Report, MAX_REPORTS> = Queue::new();
    const N: usize = 6;
    const L: usize = 3;
    static KEYS: [[&dyn Handle; N]; L] = keys!(
        [
            kc!(A), ht!(50, kb!(F), kb!(J));
            kc!(A), lyoo!(1, 0);
            kc!(A), lytp!(1);
        ],
        [
            kc!(B), kc!(B);
            kc!(B), kc!(B);
            lytp!(2), lytp!(0);
        ],
        [
            kc!(C), kc!(C);
            kc!(C), kc!(C);
            kc!(C), lytp!(0);
        ]
    );

    static CHORD1: Chord<L> = chrd!(1, 2, [Some(&kc!(Q)), None, None]);
    static CHORD2: Chord<L> = chrd!(2, 4, [Some(&lytp!(1)), None, None]);
    static KH: KeyProcessor<N, L> = KeyProcessor::new(KEYS);
    static HANDLERS: [&'static dyn Process<N, L>; 3] = [&CHORD1, &CHORD2, &KH];

    #[test]
    fn test() {
        let (producer, consumer) = unsafe { Q.split() };
        let keymap: BasicKeymap<N, L> = BasicKeymap::new(&HANDLERS, producer);

        let mut tester = Tester::new(keymap, consumer);
        tester.test(&[0], &[5], &[r!(A)]); // 1 key
        tester.test(&[0, 1], &[0, 5], &[r!(A), r!(A)]); // 2 keys
        tester.test(&[4, 4, 0], &[6, 6, 5], &[r!(A)]); // layer 0 -> 1 -> 0
        tester.test(&[5, 5, 0], &[6, 0, 5], &[r!(B)]); // layer 0 -> 1
        tester.test(&[5, 5, 0], &[6, 6, 5], &[r!(A)]); // layer 1 -> 0
        tester.test(&[5, 0], &[6, 5], &[r!(B)]); // layer 0 -> 1
        tester.test(&[5, 0], &[6, 5], &[r!(A)]); // layer 1 -> 0
        tester.test(&[4, 2, 4, 0], &[6, 6, 6, 5], &[r!(A)]); // layer 0 -> 1 -> 2 -> 0

        tester.test(&[3, 3], &[49, 5], &[r!(J)]); // tap
        tester.test(&[3], &[55], &[r!(F)]); // hold
        tester.test(&[1, 2], &[0, 5], &[r!(Q)]); // chording 1
        tester.test(&[1, 2, 1, 2, 1], &[0, 6, 0, 6, 5], &[r!(A)]); // chording 1
        tester.test(&[4, 2, 4, 2, 0], &[0, 6, 0, 6, 5], &[r!(B)]); // chording 2
    }
}
