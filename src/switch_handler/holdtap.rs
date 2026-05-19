use crate::function::Function;
use crate::switch::{SwitchEvent, Timestamp};
use crate::switch_handler::SwitchHandle;

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

impl SwitchHandle for HoldTap {
    fn handle(&self, switch_event: &SwitchEvent) -> Option<Function> {
        match switch_event {
            SwitchEvent::Pressed(i) if *i == self.thold => Some(self.hold),
            SwitchEvent::Releasing(i) if *i < self.thold => Some(self.tap),
            _ => None,
        }
    }
}

#[macro_export]
macro_rules! ht {
    ($thold:literal, $hold:expr, $tap: expr) => {
        $crate::switch_handler::holdtap::HoldTap::new($thold, $hold, $tap)
    };
}
