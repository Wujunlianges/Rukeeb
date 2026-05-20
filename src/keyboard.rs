use heapless::spsc::Producer;
use heapless::vec::Vec;

use crate::debouncer::Debouncer;
use crate::function::Function;
use crate::key::KeyEvent;
use crate::key_handler::{KeyHandle, Keymap};
use crate::report::Report;
use crate::switch::{Switch, SwitchEvent, Tick};
use crate::switch_handler::SwitchHandle;

const DT: Tick = 5;

pub struct Keyboard<'a: 'b, 'b, const N: usize> {
    layer: usize,
    debouncers: [Debouncer<DT>; N],
    switch_events: [SwitchEvent; N],
    key_layers: [usize; N],
    key_events: [Option<KeyEvent>; N],
    functions: Vec<Function<'a>, N>,
    key_handlers: &'a [&'a dyn KeyHandle<'a, N>],
    producer: Producer<'b, Report>,
}

impl<'a: 'b, 'b, const N: usize> Keyboard<'a, 'b, N> {
    pub fn new(
        key_handlers: &'a [&'a dyn KeyHandle<'a, N>],
        producer: Producer<'b, Report>,
    ) -> Keyboard<'a, 'b, N> {
        Keyboard {
            layer: 0,
            debouncers: [Debouncer::<DT>::new(); N],
            switch_events: [SwitchEvent::new(); N],
            key_layers: [0; N],
            key_events: [None; N],
            functions: Vec::new(),
            key_handlers,
            producer,
        }
    }

    pub fn tick(&mut self, signals: &[bool; N]) -> Result<(), Report> {
        self.switch_events
            .iter_mut()
            .zip(self.debouncers.iter_mut().zip(signals.iter()))
            .for_each(|(switch_event, (debouncer, signal))| {
                *switch_event = debouncer.update(*signal);
            });

        self.key_events
            .iter_mut()
            .zip(self.key_layers.iter_mut().zip(self.switch_events.iter()))
            .for_each(|(key_event, (key_layer, switch_event))| {
                if switch_event.is_pressing() {
                    *key_layer = self.layer;
                }
                if !switch_event.is_released() {
                    *key_event = Some((*key_layer, *switch_event));
                }
            });

        self.key_handlers.iter().for_each(|key_handler| {
            let _ = key_handler.handle(&mut self.key_events, &mut self.functions);
        });

        let mut reports: Vec<Report, N> = Vec::new();
        self.functions.iter().for_each(|function| match function {
            Function::Report(report) => reports.extend(report.iter().cloned()),
            Function::Layer(n_layer) => self.layer = *n_layer as usize,
        });

        while let Some(function) = self.functions.pop() {
            match function {
                Function::Report(report) => report
                    .iter()
                    .try_for_each(|r| self.producer.enqueue(r.clone()))?,
                Function::Layer(n_layer) => self.layer = n_layer as usize,
            }
        }

        Ok(())
    }
}
