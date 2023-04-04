use heapless::spsc::Producer;

use crate::action::{Action, Layer};
use crate::report::Report;

pub struct Performer {
    n_layers: usize,
    current_layer: usize,
    default_layer: usize,
    id: usize,
    reports: Producer<'static, Report, 128>,
}

impl Performer {
    pub fn new(n_layers: usize, reports: Producer<'static, Report, 128>) -> Performer {
        Performer {
            n_layers,
            current_layer: 0,
            default_layer: 0,
            id: 0,
            reports,
        }
    }

    pub fn perform(&mut self, id: usize, action: &Action) {
        match action {
            Action::Report(report) => {
                self.reports.enqueue(*report).unwrap();
            }
            Action::Layer(layer) => self.perform_layer(id, layer),
        }
    }

    fn perform_layer(&mut self, id: usize, layer: &Layer) {
        match layer {
            Layer::Current(l) => {
                if *l < self.n_layers {
                    self.current_layer = *l;
                    self.id = id;
                }
            }
            Layer::Default(l) => {
                if *l < self.n_layers {
                    self.current_layer = *l;
                    self.default_layer = *l;
                    self.id = id;
                }
            }
            Layer::UndoCurrent(l) => {
                if self.id == id && self.current_layer == *l {
                    self.current_layer = self.default_layer
                }
            }
        }
    }

    pub fn current_layer(&self) -> usize {
        self.current_layer
    }
}
