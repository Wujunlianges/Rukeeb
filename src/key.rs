use crate::switch::{SwitchEvent, Timestamp};

pub struct KeyEvent {
    switch_event: SwitchEvent,
    activation_time: Timestamp,
    n_layer: usize,
    index: usize,
}

impl KeyEvent {
    pub fn new(switch_event: SwitchEvent, state: KeyState, index: usize) -> KeyEvent {
        KeyEvent {
            switch_event,
            activation_time: state.activation_time,
            n_layer: state.n_layer,
            index,
        }
    }
}

#[derive(Clone, Copy)]
pub struct KeyState {
    activation_time: Timestamp,
    n_layer: usize,
}

impl KeyState {
    pub fn new(activation_time: Timestamp, n_layer: usize) -> KeyState {
        KeyState {
            activation_time,
            n_layer,
        }
    }
}
