use crate::action::{KeyboardAction, LayerAction};
use crate::hid_report::HIDReport;
use heapless::Vec;

pub struct Register<const L: usize, const N: usize> {
    current_layer: usize,
    default_layer: usize,
    layer_modifier_id: usize,
    hid_reports: Vec<HIDReport, 128>,
}

impl<const L: usize, const N: usize> Register<L, N> {
    pub fn new() -> Register<L, N> {
        Register {
            current_layer: 0,
            default_layer: 0,
            layer_modifier_id: 0,
            hid_reports: Vec::new(),
        }
    }

    pub fn register(&mut self, id: usize, keyboard_action: &KeyboardAction) {
        match keyboard_action {
            KeyboardAction::HIDReport(hid_report) => {
                self.hid_reports.push(*hid_report).ok();
            }
            KeyboardAction::LayerAction(layer_action) => self.register_layer(id, layer_action),
        }
    }

    fn register_layer(&mut self, id: usize, layer_action: &LayerAction) {
        match layer_action {
            LayerAction::CurrentLayer(l) => {
                if *l < L {
                    self.current_layer = *l;
                    self.layer_modifier_id = id;
                } else {
                }
            }
            LayerAction::DefaultLayer(l) => {
                if *l < L {
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
        self.hid_reports.clear();
    }

    pub fn tick(&self) -> impl Iterator<Item = &HIDReport> + '_ {
        self.hid_reports.iter()
    }

    pub fn current_layer(&self) -> usize {
        self.current_layer
    }
}

#[macro_export]
macro_rules! layer {
    ($($($v: expr),*);*) => {
        layer!(@flatten [] $($($v),*);*)
    };
    (@flatten [$($col:expr),*] $($v0:expr, $($v:expr),* );* $(;)?) => {
        layer!(@flatten [$($col,)* $($v0),*] $($($v),*);*)
    };
    (@flatten [$($col:expr),*] $($v:expr);*) => {
        [$($col,)* $($v),*]
    };
}

#[macro_export]
macro_rules! layers {
    ($([$($($v: expr),*);*]),* $(,)?) => {
        [$(layer!($($($v),*);*)),*]
    };
}
