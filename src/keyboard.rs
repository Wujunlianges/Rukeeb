use heapless::spsc::Producer;

use crate::action_handler::ActionHandler;
use crate::debouncer::Debouncer;
use crate::key::KeyEvent;
use crate::key_handler::HandleKeyEvent;
use crate::report::Report;
use crate::switch::{Switch, SwitchEvent, Tick};

const DT: Tick = 5;

pub struct Keyboard<'a: 'b, 'b, const N: usize> {
    debouncers: [Debouncer<DT>; N], // todo: this should be a trait
    switch_events: [SwitchEvent; N],
    key_layers: [usize; N],
    key_events: [Option<KeyEvent>; N],
    key_handlers: &'a [&'a dyn HandleKeyEvent<'a, N>],
    action_handler: ActionHandler<'b>,
}

impl<'a: 'b, 'b, const N: usize> Keyboard<'a, 'b, N> {
    pub fn new(
        key_handlers: &'a [&'a dyn HandleKeyEvent<'a, N>],
        producer: Producer<'b, Report>,
    ) -> Keyboard<'a, 'b, N> {
        Keyboard {
            debouncers: [Debouncer::<DT>::new(); N],
            switch_events: [SwitchEvent::new(); N],
            key_layers: [0; N],
            key_events: [None; N],
            key_handlers,
            action_handler: ActionHandler::new(producer),
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
                    *key_layer = self.action_handler.get_layer();
                }
                if !switch_event.is_released() {
                    *key_event = Some((*key_layer, *switch_event));
                }
            });

        self.key_handlers.iter().try_for_each(|key_handler| {
            key_handler.handle(&mut self.key_events, &mut self.action_handler)?;
            Ok(())
        })
    }
}
