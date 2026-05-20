use crate::action::Action;
use crate::switch::{SwitchEvent, Tick};
use crate::switch_handler::HandleSwitchEvent;

pub struct HoldTap<'a> {
    thold: Tick,
    hold: Action<'a>,
    tap: Action<'a>,
}

impl<'a> HoldTap<'a> {
    pub const fn new(thold: Tick, hold: Action<'a>, tap: Action<'a>) -> HoldTap<'a> {
        HoldTap { thold, hold, tap }
    }
}

impl<'a> HandleSwitchEvent<'a> for HoldTap<'a> {
    fn handle(&self, switch_event: &SwitchEvent) -> Option<Action<'a>> {
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
