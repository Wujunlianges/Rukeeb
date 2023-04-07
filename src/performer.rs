use heapless::spsc::Producer;

use crate::action::{Act, Action, Layer};
use crate::report::Report;

pub struct Performer<const L: usize> {
    current_layer: usize,
    default_layer: usize,
    modifier: usize,
    reports: Producer<'static, Report, 128>,
}

impl<const L: usize> Performer<L> {
    pub fn new(reports: Producer<'static, Report, 128>) -> Performer<L> {
        Performer {
            current_layer: 0,
            default_layer: 0,
            modifier: 0,
            reports,
        }
    }

    pub fn perform(&mut self, action: &Action) {
        match action {
            Action::Report(report) => {
                self.reports.enqueue(*report).unwrap();
            }
            Action::Layer(layer) => match layer {
                Layer::Current(l) => {
                    if *l < L {
                        let modifier = action as *const _ as usize;
                        if self.modifier == modifier {
                            self.current_layer = self.default_layer;
                            self.modifier = 0;
                        } else {
                            self.current_layer = *l;
                            self.modifier = modifier;
                        }
                    }
                }
                Layer::Default(l) => {
                    if *l < L {
                        self.current_layer = *l;
                        self.default_layer = *l;
                    }
                }
            },
        }
    }

    pub fn current_layer(&self) -> usize {
        self.current_layer
    }
}
