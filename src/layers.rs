use crate::action::{KeyAction, LayerAction};

pub struct Layers<const L: usize, const N: usize> {
    layers: &'static [[KeyAction; N]; L],
    current_layer: usize,
    default_layer: usize,
    layer_modifier: usize,
}

impl<const L: usize, const N: usize> Layers<L, N> {
    pub fn new(layers: &'static [[KeyAction; N]; L]) -> Layers<L, N> {
        Layers {
            layers: layers,
            current_layer: 0,
            default_layer: 0,
            layer_modifier: 0,
        }
    }

    pub fn handle_layer_action(&mut self, id: usize, layer_action: LayerAction) {
        match layer_action {
            LayerAction::CurrentLayer(l) => {
                if l < L {
                    self.current_layer = l;
                    self.layer_modifier = id;
                } else {
                }
            }
            LayerAction::DefaultLayer(l) => {
                if l < L {
                    self.current_layer = l;
                    self.default_layer = l;
                    self.layer_modifier = id;
                } else {
                }
            }
            LayerAction::UndoCurrentLayer(l) => {
                if self.layer_modifier == id && self.current_layer == l {
                    self.current_layer = self.default_layer
                }
            }
        }
    }

    pub fn current_layer(&self) -> usize {
        self.current_layer
    }

    pub fn layer(&self) -> &[KeyAction] {
        &self.layers[self.current_layer]
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
pub(crate) use layer;

#[macro_export]
macro_rules! layers {
    ($([$($($v: expr),*);*]),* $(,)?) => {
        [$(layer!($($($v),*);*)),*]
    };
}
pub(crate) use layers;
