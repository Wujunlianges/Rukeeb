use crate::switch::SwitchEvent;
use crate::switch_handler::SwitchHandle;
use crate::key_handler::Process;

pub struct Chord {
    ids: (usize, usize),
    handler: &'static dyn SwitchHandle,
}

impl Chord {
    pub const fn new(ids: (usize, usize), handler: &'static dyn SwitchHandle) -> Chord {
        Chord { ids, handler }
    }
}

impl<const N: usize> Process<N> for Chord {
    fn feed_handlers(&self, handlers: &mut [Option<&'static dyn SwitchHandle>; N], events: &[SwitchEvent; N]) {
        let (id0, id1) = self.ids;
        let (e0, e1) = (events[id0], events[id1]);
        match (e0, e1) {
            (SwitchEvent::Pressing(_), SwitchEvent::Pressed(_))
            | (SwitchEvent::Pressed(_), SwitchEvent::Pressing(_))
            | (SwitchEvent::Pressing(_), SwitchEvent::Pressing(_)) => {
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
        $crate::key_handler::chord::Chord::new(($i1, $i2), $x)
    };
}
