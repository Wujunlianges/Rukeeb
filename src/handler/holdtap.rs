use crate::event::{Event, Timestamp};
use crate::function::Function;
use crate::handler::Handle;

pub struct HoldTap {
    thold: Timestamp,
    hold: Function,
    tap: Function,
}

impl HoldTap {
    pub const fn new(thold: Timestamp, hold: Function, tap: Function) -> HoldTap {
        HoldTap { thold, hold, tap }
    }
}

impl Handle for HoldTap {
    fn handle(&self, event: &Event) -> Option<Function> {
        match event {
            Event::Pressed(i) if *i == self.thold => Some(self.hold),
            Event::Releasing(i) if *i < self.thold => Some(self.tap),
            _ => None,
        }
    }
}

#[macro_export]
macro_rules! ht {
    ($thold:literal, $hold:expr, $tap: expr) => {
        $crate::handler::holdtap::HoldTap::new($thold, $hold, $tap)
    };
}
