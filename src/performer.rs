use crate::action::{KeyboardAction, LayerAction};
use crate::report::Report;
use heapless::Vec;

pub struct Performer {
    n_layers: usize,
    current_layer: usize,
    default_layer: usize,
    layer_modifier_id: usize,
    reports: Vec<Report, 128>,
}

impl Performer {
    pub fn new(n_layers: usize) -> Performer {
        Performer {
            n_layers,
            current_layer: 0,
            default_layer: 0,
            layer_modifier_id: 0,
            reports: Vec::new(),
        }
    }

    pub fn perform(&mut self, id: usize, keyboard_action: &KeyboardAction) {
        match keyboard_action {
            KeyboardAction::Report(report) => {
                self.reports.push(*report).ok();
            }
            KeyboardAction::LayerAction(layer_action) => self.perform_layer_action(id, layer_action),
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

    pub fn clear(&mut self) {
        self.reports.clear();
    }

    pub fn tick(&self) -> impl Iterator<Item = &Report> + '_ {
        self.reports.iter()
    }

    pub fn current_layer(&self) -> usize {
        self.current_layer
    }
}
