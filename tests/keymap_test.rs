mod statics;
use crate::statics::*;

use heapless::spsc::{Consumer, Queue};

use rukeeb::keymap::Keymap;
use rukeeb::report::{Keyboard, Report};

const MAX_REPORTS: usize = 128;
const DT: usize = 5;
static mut Q: Queue<Report, MAX_REPORTS> = Queue::new();

macro_rules! r {
    ($x:tt) => {
        Report::Keyboard(Keyboard::$x)
    };
}

struct Tester<const N: usize, const L: usize> {
    keymap: Keymap<N, L>,
    consumer: Consumer<'static, Report, MAX_REPORTS>,
}

impl<const N: usize, const L: usize> Tester<N, L> {
    pub fn new(
        keymap: Keymap<N, L>,
        consumer: Consumer<'static, Report, MAX_REPORTS>,
    ) -> Tester<N, L> {
        Tester { keymap, consumer }
    }

    fn reset_keys(&mut self) {
        (0..128).for_each(|_| {
            self.keymap.tick(&[false; N]);
        });
        while self.consumer.ready() {
            self.consumer.dequeue();
        }
    }

    pub fn test(&mut self, ids: &[&[usize]], expected_outputs: &[Report]) {
        let mut switches = [false; N];
        let mut res = None;
        self.reset_keys();
        ids.iter().for_each(|&id| {
            for i in id {
                switches[*i] ^= true;
            }
            (0..DT).for_each(|_| {
                self.keymap.tick(&switches);
                while self.consumer.ready() {
                    res = self.consumer.dequeue();
                }
            });
            self.keymap.tick(&switches);
        });

        expected_outputs.iter().for_each(|expected_output| {
            assert_eq!(
                self.consumer.dequeue().unwrap_or(Report::Custom(123)),
                *expected_output,
                "Inputs: {:?} {:?}",
                ids,
                expected_outputs
            );
        });
    }
}

#[test]
fn test() {
    let (producer, consumer) = unsafe { Q.split() };
    let keymap: Keymap<N, L> = Keymap::new(&HANDLERS, producer);

    let mut tester = Tester::new(keymap, consumer);
    tester.test(&[&[0]], &[r!(A)]); // 1 key
    tester.test(&[&[0, 1]], &[r!(A), r!(A)]); // 2 keys
    tester.test(&[&[4], &[4], &[0]], &[r!(A)]); // layer 0 -> 1 -> 0

    tester.test(&[&[3], &[3]], &[r!(J)]); // tap
    tester.test(&[&[3], &[], &[]], &[r!(F)]); // hold
    tester.test(&[&[1, 2]], &[r!(Q)]); // chording 1
    tester.test(&[&[1, 2], &[1, 2], &[1]], &[r!(A)]); // chording 1
    tester.test(&[&[2, 4], &[2, 4], &[0]], &[r!(B)]); // chording 2
    tester.test(&[&[2, 4], &[2, 4], &[0]], &[r!(A)]); // chording 3

    tester.test(&[&[2]], &[r!(W), r!(E)]); // tapcomb
}
