mod statics;
use crate::statics::*;

use rukeeb::keymap::Keymap;
use rukeeb::report::Report;
use rukeeb::rpt as r;

const DT: usize = 5;

struct Tester<const N: usize, const L: usize> {
    keymap: Keymap<N, L>,
}

impl<const N: usize, const L: usize> Tester<N, L> {
    pub fn new(keymap: Keymap<N, L>) -> Tester<N, L> {
        Tester { keymap }
    }

    pub fn test(&mut self, ids: &[&[usize]], expected_outputs: &[Report]) {
        let mut switches = [false; N];
        let mut res: Vec<Report> = vec![];

        (0..128).for_each(|_| {
            self.keymap.tick(&[false; N]);
        });

        ids.iter().for_each(|&id| {
            for i in id {
                switches[*i] ^= true;
            }
            (0..DT + 1).for_each(|_| {
                let reports = self.keymap.tick(&switches);
                res = reports
                    .into_iter()
                    .filter_map(|s| s)
                    .flat_map(|s| s.iter().copied())
                    .collect::<Vec<_>>();
            });
        });

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

    tester.test(&[&[2], &[2], &[3]], &[r!(A), r!(B), r!(C)]);
    tester.test(&[&[5], &[5], &[0]], &[r!(A)]);
}
