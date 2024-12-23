use core::cmp::{max, min};
use core::sync::atomic::{AtomicBool, Ordering};

use crate::event::Event;
use crate::function::Function;
use crate::handler::Handle;
use crate::processor::Process;

pub struct Chord<const L: usize> {
    ids: (usize, usize),
    handlers: [Option<&'static dyn Handle>; L],
}

impl<const L: usize> Chord<L> {
    pub const fn new(ids: (usize, usize), handlers: [Option<&'static dyn Handle>; L]) -> Chord<L> {
        Chord { ids, handlers }
    }
}

impl<const N: usize, const L: usize> Process<N, L> for Chord<L> {
    fn process(
        &self,
        handlers: &mut [Option<&'static dyn Handle>; N],
        events: &[Event; N],
        layer: usize,
    ) {
        let (id0, id1) = self.ids;
        let event0 = events[id0];
        let event1 = events[id1];
        if let Some(handler) = self.handlers[layer] {
            match (event0, event1) {
                (Event::Pressed(_), Event::Pressed(_))
                | (Event::Pressed(_), Event::Press(_))
                | (Event::Press(_), Event::Pressed(_))
                | (Event::Press(_), Event::Press(_)) => {
                    handlers[id0] = Some(handler);
                    handlers[id1] = Some(handler);
                }
                (Event::Release(_), Event::Pressed(_)) | (Event::Pressed(_), Event::Release(_)) => {
                    handlers[id0] = None;
                    handlers[id1] = None;
                }
                _ => {}
            }
        }
    }
}

#[macro_export]
macro_rules! chrd {
    ($i1:literal, $i2:literal, [$($x:expr),* $(,)?]) => {
        $crate::processor::chord::Chord::new(($i1, $i2), [$($x),*])
    };
}
