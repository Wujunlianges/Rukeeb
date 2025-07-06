use crate::event::Event;
use crate::function::Function;
use crate::handler::Handle;

pub struct TapComb(&'static [Function]);

impl TapComb {
    pub const fn new(fs: &'static [Function]) -> TapComb {
        TapComb(fs)
    }
}

impl Handle for TapComb {
    fn handle(&self, event: &Event) -> Option<&[Function]> {
        match event {
            Event::Pressing(_) => Some(self.0),
            _ => None,
        }
    }
}

#[macro_export]
macro_rules! tc {
    ($fs: expr) => {
        $crate::handler::tapcomb::TapComb::new(&$fs)
    };
}
