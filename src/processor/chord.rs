use crate::event::Event;
use crate::handler::Handle;
use crate::processor::Process;

pub struct Chord {
    ids: (usize, usize),
    handler: &'static dyn Handle,
}

impl Chord {
    pub const fn new(ids: (usize, usize), handler: &'static dyn Handle) -> Chord {
        Chord { ids, handler }
    }
}

impl<const N: usize> Process<N> for Chord {
    fn process(&self, handlers: &mut [Option<&'static dyn Handle>; N], events: &[Event; N]) {
        let (id0, id1) = self.ids;
        let (e0, e1) = (events[id0], events[id1]);
        match (e0, e1) {
            (Event::Pressing(_), Event::Pressed(_))
            | (Event::Pressed(_), Event::Pressing(_))
            | (Event::Pressing(_), Event::Pressing(_)) => {
                handlers[id0] = Some(self.handler);
                handlers[id1] = Some(self.handler);
            }
            _ => {}
        }
    }
}

#[macro_export]
macro_rules! chrd {
    (($i1:literal, $i2:literal), $x:expr) => {
        $crate::processor::chord::Chord::new(($i1, $i2), $x)
    };
}
