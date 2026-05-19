use heapless::Vec;
use heapless::spsc::Producer;

use crate::debouncer::Debouncer;
use crate::function::Function;
use crate::key::{KeyEvent, KeyState};
use crate::key_handler::Process;
use crate::report::Report;
use crate::switch::{Switch, SwitchEvent, Timestamp};
use crate::switch_handler::SwitchHandle;

const DT: Timestamp = 5;

pub struct Keymap<const N: usize> {
    time: Timestamp,
    n_layer: usize,
    debouncers: [Debouncer<DT>; N],
    switch_events: [SwitchEvent; N],
    key_states: [Option<KeyState>; N],
    key_events: Vec<KeyEvent, N>,
    handlers: [Option<&'static dyn SwitchHandle>; N],
    layers: &'static [&'static [&'static dyn Process<N>]],
}

impl<const N: usize> Keymap<N> {
    pub fn new(layers: &'static [&'static [&'static dyn Process<N>]]) -> Keymap<N> {
        Keymap {
            time: 0,
            n_layer: 0,
            debouncers: [Debouncer::<DT>::new(); N],
            switch_events: [SwitchEvent::new(); N],
            key_states: [None; N],
            key_events: Vec::new(),
            handlers: [None; N],
            layers,
        }
    }

    pub fn tick(&mut self, signals: &[bool; N]) -> [Option<&[Report]>; N] {
        self.switch_events
            .iter_mut()
            .zip(self.debouncers.iter_mut().zip(signals.iter()))
            .for_each(|(switch_event, (debouncer, signal))| {
                *switch_event = debouncer.update(*signal);
            });

        self.key_events.extend(
            self.key_states
                .iter()
                .zip(self.switch_events)
                .enumerate()
                .filter_map(|(i, (key_state, switch_event))| {
                    key_state.map(|key_state| KeyEvent::new(switch_event, key_state, i))
                }),
        );

        // Process all events.
        self.layers[self.n_layer]
            .iter()
            .for_each(|processor| processor.feed_handlers(&mut self.handlers, &self.switch_events));

        // Handle individual events.
        let mut reports: [Option<&[Report]>; N] = [None; N];
        reports
            .iter_mut()
            .zip(self.handlers.iter_mut().zip(self.switch_events.iter()))
            .for_each(|(report, (handler, switch_event))| {
                if let Some(h) = handler {
                    if let Some(function) = h.handle(switch_event) {
                        match function {
                            Function::Report(r) => *report = Some(r),
                            Function::Layer(l) => self.n_layer = l as usize,
                        }
                    }
                    if matches!(switch_event, SwitchEvent::Released(_)) {
                        *handler = None;
                    }
                }
            });

        self.key_events.clear();
        self.key_states
            .iter_mut()
            .zip(self.switch_events)
            .for_each(|(key_state, switch_event)| match switch_event {
                SwitchEvent::Pressed(_) | SwitchEvent::Released(_) => {}
                SwitchEvent::Pressing(_) => {
                    *key_state = Some(KeyState::new(self.time, self.n_layer))
                }
                SwitchEvent::Releasing(_) => *key_state = None,
            });

        if self.key_states.iter().all(|key_state| key_state.is_none()) {
            self.time = 0;
        } else {
            self.time = self.time.saturating_add(1);
        }

        reports
    }
}
