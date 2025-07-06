mod statics;
use crate::statics::*;

use rukeeb::keymap::Keymap;
use rukeeb::report::{Keyboard, Report};

const DT: usize = 5;

macro_rules! r {
    ($x:tt) => {
        Report::Keyboard(Keyboard::$x)
    };
}

struct Tester<const N: usize, const L: usize> {
    keymap: Keymap<N, L>,
}

impl<const N: usize, const L: usize> Tester<N, L> {
    pub fn new(keymap: Keymap<N, L>) -> Tester<N, L> {
        Tester { keymap }
    }

    pub fn test(&mut self, ids: &[&[usize]], expected_outputs: &[Report]) {
        let mut switches = [false; N];
        let mut res = [None; N];

        (0..128).for_each(|_| {
            self.keymap.tick(&[false; N]);
        });

        ids.iter().for_each(|&id| {
            for i in id {
                switches[*i] ^= true;
            }
            (0..DT + 1).for_each(|_| {
                res = self.keymap.tick(&switches);
            });
        });
        let res = res.into_iter().filter_map(|r| r).collect::<Vec<_>>();
        assert_eq!(
            res, expected_outputs,
            "Inputs: {:?} {:?}",
            ids, expected_outputs
        );
    }
}

#[test]
fn test() {
    let keymap: Keymap<N, L> = Keymap::new(&HANDLERS);

    let mut tester = Tester::new(keymap);
    tester.test(&[&[0]], &[r!(A)]); // 1 key
    tester.test(&[&[0, 1]], &[r!(A), r!(A)]); // 2 keys
    tester.test(&[&[4], &[4], &[0]], &[r!(A)]); // layer 0 -> 1 -> 0

    tester.test(&[&[3], &[3]], &[r!(J)]); // tap
    tester.test(&[&[3], &[], &[]], &[r!(F)]); // hold
    tester.test(&[&[1, 2]], &[r!(Q), r!(Q)]); // chording 1
    tester.test(&[&[1, 2], &[1, 2], &[1]], &[r!(A)]); // chording 1
    tester.test(&[&[2, 4], &[2, 4], &[0]], &[r!(B)]); // chording 2
    tester.test(&[&[2, 4], &[2, 4], &[0]], &[r!(A)]); // chording 3
}
