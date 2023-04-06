use core::sync::atomic::{AtomicBool, Ordering};

use crate::action::{Act, Action};
use crate::event::Event;
use crate::handler::Handle;
use crate::performer::Performer;

pub struct Chord<const L: usize> {
    is_triggered: AtomicBool,
    ids: (usize, usize),
    actions: [Option<Action>; L],
}

impl<const L: usize> Chord<L> {
    pub const fn new(ids: (usize, usize), actions: [Option<Action>; L]) -> Chord<L> {
        Chord {
            is_triggered: AtomicBool::new(false),
            ids,
            actions,
        }
    }
}

impl<const L: usize> Handle for Chord<L> {
    fn handle(&self, id: usize, events: &mut [Event], performer: &mut Performer) {
        let (id0, id1) = self.ids;

        if let Some(action) = &self.actions[performer.current_layer()] {
            match self.is_triggered.load(Ordering::Relaxed) {
                false => match (events[id0], events[id1]) {
                    (Event::Press(_), Event::Pressed(_))
                    | (Event::Pressed(_), Event::Press(_))
                    | (Event::Press(_), Event::Press(_)) => {
                        self.is_triggered.store(true, Ordering::Relaxed);
                        performer.perform(id, action);
                        events[id0] = Event::Released(0);
                        events[id1] = Event::Released(0);
                    }
                    _ => {}
                },
                true => {
                    match (events[id0], events[id1]) {
                        (Event::Release(_), Event::Released(_))
                        | (Event::Released(_), Event::Release(_))
                        | (Event::Release(_), Event::Release(_)) => {
                            self.is_triggered.store(false, Ordering::Relaxed);
                        }
                        _ => {}
                    }
                    events[id0] = Event::Released(0);
                    events[id1] = Event::Released(0);
                }
            }
        }
    }
}

#[macro_export]
macro_rules! chrd {
    ($i1:literal, $i2:literal, [$($x:expr),* $(,)?]) => {
        $crate::handler::chord::Chord::new(($i1, $i2), [$($x),*])
    };
}
