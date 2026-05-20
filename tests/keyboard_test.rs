mod statics;
use crate::statics::*;

use rukeeb::keyboard::Keyboard;
use rukeeb::report::Report;
use rukeeb::rpt as r;

type Tick<'a> = (usize, &'a [usize], &'a [Report]);

struct Tester<const N: usize> {
    signals: [bool; N],
    keyboard: Keyboard<N>,
}

impl<const N: usize> Tester<N> {
    pub fn new(keyboard: Keyboard<N>) -> Tester<N> {
        Tester {
            signals: [false; N],
            keyboard,
        }
    }

    pub fn tick(&mut self) -> impl IntoIterator<Item = Report> {
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
            self.keyboard.tick(&self.signals);
        });
    }

    pub fn test<'a>(&mut self, ticks: &[Tick<'a>]) {
        self.reset();

        let tick_vec: Vec<Option<(&[usize], &[Report])>> = ticks.iter().fold(
            vec![None; ticks[ticks.len() - 1].0 + 1],
            |mut acc, (time, input, expected_output)| {
                acc[*time] = Some((*input, *expected_output));
                acc
            },
        );
        let mut timer = 0;
        tick_vec.into_iter().for_each(|tick| {
            if let Some((input, _)) = tick {
                self.update(input);
            }
            let output: Vec<Report> = self.tick().into_iter().collect();
            if let Some((_, expected_output)) = tick {
                assert!(output.eq(&expected_output), "Error at time {}", timer);
            }
            timer += 1;
        });
    }
}

#[test]
fn test() {
    let keyboard: Keyboard<N> = Keyboard::new(&KEY_HANDLERS);

    let mut tester = Tester::new(keyboard);

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
