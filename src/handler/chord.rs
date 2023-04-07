use core::sync::atomic::{AtomicBool, Ordering};

use crate::action::Action;
use crate::event::Event;
use crate::handler::Handle;
use crate::performer::Performer;
use crate::state::State;

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
    fn handle(&self, states: &mut [State; N], performer: &mut Performer<L>) {
        let (id0, id1) = self.ids;

        let State(enabled0, layer0, event0) = states[id0];
        let State(enabled1, layer1, event1) = states[id1];

        if enabled0 && enabled1 && (layer0 == layer1) {
            let layer = layer0;
            if let Some(action) = &self.actions[layer] {
                match self.triggered.load(Ordering::Relaxed) {
                    false => match (event0, event1) {
                        (Event::Press(_), Event::Pressed(_))
                        | (Event::Pressed(_), Event::Press(_))
                        | (Event::Press(_), Event::Press(_)) => {
                            self.triggered.store(true, Ordering::Relaxed);
                            states[id0].disable();
                            states[id1].disable();
                            performer.perform(action);
                        }
                        _ => {}
                    },
                    true => {
                        states[id0].disable();
                        states[id1].disable();
                        match (event0, event1) {
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
