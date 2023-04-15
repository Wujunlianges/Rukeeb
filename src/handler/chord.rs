use core::cmp::{max, min};
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

        let State(enabled0, event0, layer0) = states[id0];
        let State(enabled1, event1, layer1) = states[id1];

        if enabled0 && enabled1 && (layer0 == layer1) {
            match self.triggered.load(Ordering::Relaxed) {
                false => {
                    if let Event::Press(_) = max(&event0, &event1) {
                        if let Some(action) = &self.actions[layer0] {
                            self.triggered.store(true, Ordering::Relaxed);
                            states[id0].disable();
                            states[id1].disable();
                            performer.perform(action);
                        }
                    }
                }
                true => {
                    states[id0].disable();
                    states[id1].disable();
                    if let Event::Release(_) = min(&event0, &event1) {
                        self.triggered.store(false, Ordering::Relaxed);
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
