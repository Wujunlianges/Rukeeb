use crate::action::Action;
use crate::switch::SwitchEvent;

mod holdtap;
pub use holdtap::HoldTap;

pub trait HandleSwitchEvent: Sync {
    fn handle(&self, switch_event: &SwitchEvent) -> Option<Action>;
}

pub struct Hold(Action);
pub struct Tap(Action);
pub struct OnOff(Action, Action);

impl Hold {
    pub const fn new(f: Action) -> Hold {
        Hold(f)
    }
}

impl HandleSwitchEvent for Hold {
    fn handle(&self, switch_event: &SwitchEvent) -> Option<Action> {
        match switch_event {
            SwitchEvent::Pressing(_) | SwitchEvent::Pressed(_) => Some(self.0),
            _ => None,
        }
    }
}

impl Tap {
    pub const fn new(f: Action) -> Tap {
        Tap(f)
    }
}

impl HandleSwitchEvent for Tap {
    fn handle(&self, switch_event: &SwitchEvent) -> Option<Action> {
        match switch_event {
            SwitchEvent::Pressing(_) => Some(self.0),
            _ => None,
        }
    }
}

impl OnOff {
    pub const fn new(f0: Action, f1: Action) -> OnOff {
        OnOff(f0, f1)
    }
}

impl HandleSwitchEvent for OnOff {
    fn handle(&self, switch_event: &SwitchEvent) -> Option<Action> {
        match switch_event {
            SwitchEvent::Pressing(_) => Some(self.0),
            SwitchEvent::Releasing(_) => Some(self.1),
            _ => None,
        }
    }
}

// Keyboard Hold
#[macro_export]
macro_rules! kh {
    ($x: tt) => {
        $crate::switch_handler::Hold::new($crate::action::Action::Report($crate::rpt!($x)))
    };
}

// Keyboard Tap
#[macro_export]
macro_rules! kt {
    ($x: tt) => {
        $crate::switch_handler::Tap::new($crate::action::Action::Report($crate::rpt!($x)))
    };
}

// Layer Tap
#[macro_export]
macro_rules! lt {
    ($x:tt) => {
        $crate::switch_handler::Tap::new($crate::action::Action::Layer($x))
    };
}

// Layer OnOff
#[macro_export]
macro_rules! lo {
    ($x0:tt, $x1:tt) => {
        $crate::switch_handler::OnOff::new(
            $crate::action::Action::Layer($x0),
            $crate::action::Action::Layer($x1),
        )
    };
}
