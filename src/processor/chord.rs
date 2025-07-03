use crate::event::Event;
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
        let (e0, e1) = (events[id0], events[id1]);
        if let Some(handler) = self.handlers[layer] {
            match (e0, e1) {
                (Event::Pressing(_), Event::Pressed(_))
                | (Event::Pressed(_), Event::Pressing(_))
                | (Event::Pressing(_), Event::Pressing(_)) => {
                    handlers[id0] = Some(handler);
                    handlers[id1] = Some(handler);
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
