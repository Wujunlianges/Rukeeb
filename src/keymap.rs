use heapless::spsc::Producer;

use crate::debouncer::{Debounce, Debouncer};
use crate::event::Event;
use crate::function::Function;
use crate::handler::Handle;
use crate::processor::Process;
use crate::report::Report;

const MAX_REPORTS: usize = 128;
const DT: u8 = 5;

pub struct Keymap<const N: usize, const L: usize> {
    layer: usize,
    debouncers: [Debouncer<DT>; N],
    events: [Event; N],
    handlers: [Option<&'static dyn Handle>; N],
    processors: &'static [&'static [&'static dyn Process<N>]; L],
    reporter: Producer<'static, Report, MAX_REPORTS>,
}

impl<const N: usize, const L: usize> Keymap<N, L> {
    pub fn new(
        processors: &'static [&'static [&'static dyn Process<N>]; L],
        reporter: Producer<'static, Report, MAX_REPORTS>,
    ) -> Keymap<N, L> {
        Keymap {
            layer: 0,
            debouncers: [Debouncer::<DT>::new(); N],
            events: [Event::new(); N],
            handlers: [None; N],
            processors,
            reporter,
        }
    }

    pub fn tick(&mut self, switches: &[bool; N]) {
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
        self.handlers
            .iter_mut()
            .zip(self.events.iter())
            .for_each(|(handler, event)| {
                if let Some(Some(functions)) = handler.map(|h| h.handle(event)) {
                    for function in functions {
                        match function {
                            Function::Report(report) => self.reporter.enqueue(*report).unwrap(),
                            Function::Layer(layer) => self.layer = *layer as usize,
                        }
                    }
                }
                if matches!(event, Event::Released(_)) {
                    *handler = None;
                }
            });
    }
}
