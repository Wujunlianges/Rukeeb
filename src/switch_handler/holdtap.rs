use crate::action::Action;
use crate::switch::{SwitchEvent, Tick};
use crate::switch_handler::HandleSwitchEvent;

pub struct HoldTap {
    thold: Tick,
    hold: Action,
    tap: Action,
}

impl HoldTap {
    pub const fn new(thold: Tick, hold: Action, tap: Action) -> HoldTap {
        HoldTap { thold, hold, tap }
    }
}

impl HandleSwitchEvent for HoldTap {
    fn handle(&self, switch_event: &SwitchEvent) -> Option<Action> {
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
        $crate::switch_handler::HoldTap::new($thold, $hold, $tap)
    };
}
