mod statics;

use crate::statics::*;
use heapless::spsc::{Consumer, Queue};

use rukeeb::debouncer::{Debounce, ThresholdDebouncer};
use rukeeb::key_handler::HandleKeyEvent;
use rukeeb::keyboard::Keyboard;
use rukeeb::report::Report;
use rukeeb::rpt as r;

type TestCase<'a> = (usize, &'a [usize], &'a [Report]);

struct Tester<'a: 'b, 'b, const N: usize> {
    signals: [bool; N],
    keyboard: Keyboard<'a, 'b, N>,
    consumer: Consumer<'b, Report>,
}

impl<'a: 'b, 'b, const N: usize> Tester<'a, 'b, N> {
    pub fn new(
        debouncer: &'b mut dyn Debounce<N>,
        key_handlers: &'a [&'a dyn HandleKeyEvent<'a, N>],
        queue: &'b mut Queue<Report, 128>,
    ) -> Tester<'a, 'b, N> {
        let (producer, consumer) = queue.split();
        Tester {
            signals: [false; N],
            keyboard: Keyboard::new(debouncer, key_handlers, producer),
            consumer,
        }
    }

    pub fn tick(&mut self) -> Result<(), Report> {
        self.keyboard.tick(&self.signals)
    }

    pub fn update(&mut self, signals: &[usize]) {
        signals.iter().for_each(|i| {
            self.signals[*i] = !self.signals[*i];
        });
    }

    pub fn reset(&mut self) {
        self.signals.iter_mut().for_each(|s| *s = false);
        (0..5).for_each(|_| {
            let _ = self.keyboard.tick(&self.signals);
        });
        while let Some(_) = self.consumer.dequeue() {}
    }

    pub fn test(&mut self, test_cases: &[TestCase]) {
        self.reset();

        let test_cases: Vec<Option<(&[usize], &[Report])>> = test_cases.iter().fold(
            vec![None; test_cases[test_cases.len() - 1].0 + 1],
            |mut acc, (time, input, expected_output)| {
                acc[*time] = Some((*input, *expected_output));
                acc
            },
        );
        let mut timer = 0;
        test_cases.into_iter().for_each(|test_case| {
            test_case.map(|(input, _)| self.update(input));
            let _ = self.tick();
            let mut output = Vec::new();
            while let Some(report) = self.consumer.dequeue() {
                output.push(report);
            }

            test_case.map(|(_, expected_output)| {
                let mut expected_output = expected_output.to_vec();
                output.sort();
                expected_output.sort();
                assert_eq!(output, expected_output, "Timestamp: {:?}", timer);
            });

            timer += 1;
        });
    }
}

#[test]
fn test() {
    let mut queue = Queue::<Report, 128>::new();
    let mut debouncer = ThresholdDebouncer::<N, 5>::new();

    let mut tester = Tester::new(&mut debouncer, &KEY_HANDLERS, &mut queue);

    // DT is 5

    // basic
    tester.test(&[
        (0, &[0], &[]), // 0 activates at 0 + DT
        (5, &[], &[r!(A)]),
        (6, &[0], &[r!(A)]), // 0 deactivates at 6 + DT
        (11, &[], &[]),
    ]);

    // tap
    tester.test(&[
        (0, &[0, 3], &[]), // 0 activates at 0 + DT
        (5, &[], &[r!(A)]),
        (7, &[0, 3], &[r!(A)]), // 0 deactivates at 7 + DT; 3 is a tap, activates at 7 + DT
        (11, &[], &[r!(A)]),
        (12, &[], &[r!(J)]),
    ]);

    // hold
    tester.test(&[
        (0, &[0, 3], &[]), // 0 activates at 0 + DT
        (5, &[], &[r!(A)]),
        (8, &[0, 3], &[r!(A)]), // 0 deactivates at 8 + DT; 3 is a hold, activates at 0 + 7 + DT
        (12, &[], &[r!(A), r!(F)]),
        (13, &[], &[]),
    ]);

    // chord
    tester.test(&[
        (0, &[1, 2], &[]), // 1+2 activates at 0 + DT
        (5, &[], &[r!(Q), r!(Q)]),
        (6, &[1, 2], &[r!(Q), r!(Q)]), // 1+2 deactivates at 6 + DT
        (11, &[], &[]),
        (12, &[1], &[]), // 1 activates at 12 + DT
        (17, &[], &[r!(A)]),
    ]);

    // chord + layer + multi-reports
    tester.test(&[
        (0, &[2, 4], &[]), // 2+4 activates at 0 + DT
        (6, &[2, 4], &[]), // 2+4 deactivates at 6 + DT | level 1
        (11, &[0], &[]),   // 0 activates at 11 + DT
        (16, &[], &[r!(B)]),
        (17, &[0, 2], &[r!(B)]), // 0 deactivates and 2 activates at 17 + DT
        (23, &[2, 3], &[]),      // 2 deactivates and 3 activates at 22 + DT | level 2
        (28, &[], &[r!(A), r!(B), r!(C)]),
        (29, &[3, 5], &[r!(A), r!(B), r!(C)]), // 3 deactivates and 5 activates at 29 + DT
        (35, &[0, 5], &[]),                    // 0 activates and 5 deactivates at 35 + DT
        (40, &[], &[r!(A)]),
        (41, &[0], &[r!(A)]), // 0 deactivates at 41 + DT
        (46, &[], &[]),
    ]);
}
