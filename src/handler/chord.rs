use core::sync::atomic::{AtomicBool, Ordering};

use crate::action::Action;
use crate::event::Event;
use crate::handler::Handle;
use crate::performer::Performer;

pub struct Chord<const L: usize> {
    triggered: AtomicBool,
    ids: (usize, usize),
    actions: [Option<Action>; L],
}

impl<const L: usize> Chord<L> {
    pub const fn new(ids: (usize, usize), actions: [Option<Action>; L]) -> Chord<L> {
        Chord {
            triggered: AtomicBool::new(false),
            ids,
            actions,
        }
    }
}

impl<const N: usize, const L: usize> Handle<N, L> for Chord<L> {
    fn handle(
        &self,
        layers: &[usize; N],
        events: &[Event; N],
        enabled: &mut [bool; N],
        performer: &mut Performer<L>,
    ) {
        let (id0, id1) = self.ids;

        if enabled[id0] && enabled[id1] && (layers[id0] == layers[id1]) {
            let layer = layers[id0];
            if let Some(action) = &self.actions[layer] {
                match self.triggered.load(Ordering::Relaxed) {
                    false => match (events[id0], events[id1]) {
                        (Event::Press(_), Event::Pressed(_))
                        | (Event::Pressed(_), Event::Press(_))
                        | (Event::Press(_), Event::Press(_)) => {
                            self.triggered.store(true, Ordering::Relaxed);
                            enabled[id0] = false;
                            enabled[id1] = false;
                            performer.perform(action);
                        }
                        _ => {}
                    },
                    true => {
                        enabled[id0] = false;
                        enabled[id1] = false;
                        match (events[id0], events[id1]) {
                            (Event::Release(_), Event::Released(_))
                            | (Event::Released(_), Event::Release(_))
                            | (Event::Release(_), Event::Release(_)) => {
                                self.triggered.store(false, Ordering::Relaxed);
                            }
                            _ => {}
                        }
                    }
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
