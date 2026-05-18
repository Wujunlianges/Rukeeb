use heapless::spsc::Producer;

use crate::debouncer::{Debounce, Debouncer};
use crate::event::{Event, Timestamp};
use crate::function::Function;
use crate::handler::Handle;
use crate::processor::Process;
use crate::report::Report;

const DT: Timestamp = 5;

pub struct Keymap<const N: usize, const L: usize> {
    layer: usize,
    debouncers: [Debouncer<DT>; N],
    events: [Event; N],
    handlers: [Option<&'static dyn Handle>; N],
    processors: &'static [&'static [&'static dyn Process<N>]; L],
}

impl<const N: usize, const L: usize> Keymap<N, L> {
    pub fn new(processors: &'static [&'static [&'static dyn Process<N>]; L]) -> Keymap<N, L> {
        Keymap {
            layer: 0,
            debouncers: [Debouncer::<DT>::new(); N],
            events: [Event::new(); N],
            handlers: [None; N],
            processors,
        }
    }

    pub fn tick(&mut self, switches: &[bool; N]) -> [Option<&[Report]>; N] {
        self.events
            .iter_mut()
            .zip(self.debouncers.iter_mut().zip(switches.iter()))
            .for_each(|(event, (debouncer, switch))| {
                *event = debouncer.debounce(*switch);
            });

        // Process all events.
        self.processors[self.layer]
            .iter()
            .for_each(|handler| handler.process(&mut self.handlers, &self.events));

        // Handle individual events.
        let mut reports: [Option<&[Report]>; N] = [None; N];
        reports
            .iter_mut()
            .zip(self.handlers.iter_mut().zip(self.events.iter()))
            .for_each(|(report, (handler, event))| {
                if let Some(h) = handler {
                    if let Some(function) = h.handle(event) {
                        match function {
                            Function::Report(r) => *report = Some(r),
                            Function::Layer(l) => self.layer = l as usize,
                        }
                    }
                    if matches!(event, Event::Released(_)) {
                        *handler = None;
                    }
                }
            });

        reports
    }
}
