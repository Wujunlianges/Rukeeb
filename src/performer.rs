use heapless::spsc::Producer;

use crate::action::{KeyboardAction, LayerAction};
use crate::report::Report;

pub struct Performer {
    n_layers: usize,
    current_layer: usize,
    default_layer: usize,
    layer_modifier_id: usize,
    reports: Producer<'static, Report, 128>,
}

impl Performer {
    pub fn new(n_layers: usize, reports: Producer<'static, Report, 128>) -> Performer {
        Performer {
            n_layers,
            current_layer: 0,
            default_layer: 0,
            layer_modifier_id: 0,
            reports,
        }
    }

    pub fn perform(&mut self, id: usize, keyboard_action: &KeyboardAction) {
        match keyboard_action {
            KeyboardAction::Report(report) => {
                self.reports.enqueue(*report).unwrap();
            }
            KeyboardAction::LayerAction(layer_action) => {
                self.perform_layer_action(id, layer_action)
            }
        }
    }

    fn perform_layer_action(&mut self, id: usize, layer_action: &LayerAction) {
        match layer_action {
            LayerAction::CurrentLayer(l) => {
                if *l < self.n_layers {
                    self.current_layer = *l;
                    self.layer_modifier_id = id;
                } else {
                }
            }
            LayerAction::DefaultLayer(l) => {
                if *l < self.n_layers {
                    self.current_layer = *l;
                    self.default_layer = *l;
                    self.layer_modifier_id = id;
                } else {
                }
            }
            LayerAction::UndoCurrentLayer(l) => {
                if self.layer_modifier_id == id && self.current_layer == *l {
                    self.current_layer = self.default_layer
                }
            }
        }
    }

    pub fn current_layer(&self) -> usize {
        self.current_layer
    }
}
