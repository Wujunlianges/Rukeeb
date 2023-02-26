use crate::action::{Action, KeyboardAction};
use crate::event::Event;
use crate::register::Register;

pub trait Handle<const L: usize, const N: usize>: Sync {
    fn handle(&self, events: &mut [Event], register: &mut Register<L, N>);
}

pub struct Handler<const L: usize, const N: usize>(pub &'static dyn Handle<L, N>);

impl<const L: usize, const N: usize> Handler<L, N> {
    pub fn handle(&self, events: &mut [Event], register: &mut Register<L, N>) {
        self.0.handle(events, register);
    }
}

pub struct Chord<const L: usize> {
    pub keys: (usize, usize),
    pub key_actions: [Option<Action>; L],
}

impl<const L: usize, const N: usize> Handle<L, N> for Chord<L> {
    fn handle(&self, events: &mut [Event], register: &mut Register<L, N>) {
        let key0 = self.keys.0;
        let key1 = self.keys.1;

        match (events[key0], events[key1]) {
            (Event::Released(_), _)
            | (_, Event::Released(_))
            | (Event::Release(_), Event::Release(_)) => {}
            (Event::Pressed(_), event)
            | (event, Event::Pressed(_))
            | (Event::Press(_), event)
            | (event, Event::Press(_)) => {
                events[key0] = Event::Released(0);
                events[key1] = Event::Released(0);
                if let Some(key_action) = &self.key_actions[register.current_layer()] {
                    if let Some(keyboard_action) = key_action.event(&event) {
                        register.keyboard_action(
                            (key0 + key1) * (key0 + key1 + 1) / 2 + key1 + N,
                            &keyboard_action,
                        );
                    }
                }
            }
        }
    }
}

pub struct Comb<const L: usize> {
    pub key: usize,
    pub keyboard_actions: [Option<&'static [KeyboardAction]>; L],
}

impl<const L: usize, const N: usize> Handle<L, N> for Comb<L> {
    fn handle(&self, events: &mut [Event], register: &mut Register<L, N>) {
        match events[self.key] {
            Event::Pressed(_) | Event::Released(_) => events[self.key] = Event::Released(0),
            Event::Press(_) => {
                if let Some(keyboard_actions) = self.keyboard_actions[register.current_layer()] {
                    for keyboard_action in keyboard_actions {
                        register.keyboard_action(self.key, keyboard_action);
                    }
                }
                events[self.key] = Event::Released(0);
            }
            _ => {}
        }
    }
}
