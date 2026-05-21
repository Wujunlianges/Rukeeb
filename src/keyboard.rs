use heapless::spsc::Producer;

use crate::action_handler::ActionHandler;
use crate::debouncer::Debounce;
use crate::key::KeyEvent;
use crate::key_handler::HandleKeyEvent;
use crate::report::Report;
use crate::switch::SwitchEvent;

pub struct Keyboard<'a: 'b, 'b, const N: usize> {
    debouncer: &'b mut dyn Debounce<N>,
    key_handlers: &'a [&'a dyn HandleKeyEvent<'a, N>],
    action_handler: ActionHandler<'b>,
    switch_events: [SwitchEvent; N],
    key_layers: [usize; N],
    key_events: [Option<KeyEvent>; N],
}

impl<'a: 'b, 'b, const N: usize> Keyboard<'a, 'b, N> {
    pub fn new(
        debouncer: &'b mut dyn Debounce<N>,
        key_handlers: &'a [&'a dyn HandleKeyEvent<'a, N>],
        producer: Producer<'b, Report>,
    ) -> Keyboard<'a, 'b, N> {
        Keyboard {
            debouncer,
            key_handlers,
            action_handler: ActionHandler::new(producer),
            switch_events: [SwitchEvent::new(); N],
            key_layers: [0; N],
            key_events: [None; N],
        }
    }

    pub fn tick(&mut self, signals: &[bool; N]) -> Result<(), Report> {
        self.debouncer.debounce(signals, &mut self.switch_events);

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
