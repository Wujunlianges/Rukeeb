use crate::action::KeyboardAction;
use crate::event::Event;
use crate::performer::Performer;

pub trait Handle: Sync {
    fn handle(&self, id: usize, events: &mut [Event], performer: &mut Performer);
}

pub struct Handler(pub &'static dyn Handle);

impl Handler {
    pub fn handle(&self, id: usize, events: &mut [Event], performer: &mut Performer) {
        self.0.handle(id, events, performer);
    }
}

pub struct Chord<const L: usize> {
    pub keys: (usize, usize),
    pub keyboard_actions: [Option<KeyboardAction>; L],
}

impl<const L: usize> Handle for Chord<L> {
    fn handle(&self, id: usize, events: &mut [Event], performer: &mut Performer) {
        let key0 = self.keys.0;
        let key1 = self.keys.1;

        match (events[key0], events[key1]) {
            (Event::Released(_), _) | (_, Event::Released(_)) => {}
            (Event::Press(_), _) | (_, Event::Press(_)) => {
                events[key0] = Event::Released(0);
                events[key1] = Event::Released(0);
                if let Some(keyboard_action) = &self.keyboard_actions[performer.current_layer()] {
                    performer.perform(id, &keyboard_action);
                }
            }
            (Event::Pressed(_), _) | (_, Event::Pressed(_)) => {
                events[key0] = Event::Released(0);
                events[key1] = Event::Released(0);
            }
            (Event::Release(_), Event::Release(_)) => {}
        }
    }
}

pub struct Comb<const L: usize> {
    pub key: usize,
    pub keyboard_actions: [Option<&'static [KeyboardAction]>; L],
}

impl<const L: usize> Handle for Comb<L> {
    fn handle(&self, _: usize, events: &mut [Event], performer: &mut Performer) {
        match events[self.key] {
            Event::Pressed(_) | Event::Released(_) => events[self.key] = Event::Released(0),
            Event::Press(_) => {
                if let Some(keyboard_actions) = self.keyboard_actions[performer.current_layer()] {
                    for keyboard_action in keyboard_actions {
                        performer.perform(self.key, keyboard_action);
                    }
                }
                events[self.key] = Event::Released(0);
            }
            _ => {}
        }
    }
}
