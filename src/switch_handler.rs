use crate::action::Action;
use crate::switch::SwitchEvent;

mod holdtap;
pub use holdtap::HoldTap;

pub trait HandleSwitchEvent<'a>: Sync {
    fn handle(&self, switch_event: &SwitchEvent) -> Option<Action<'a>>;
}

pub struct Hold<'a>(Action<'a>);
pub struct Tap<'a>(Action<'a>);
pub struct OnOff<'a>(Action<'a>, Action<'a>);

impl<'a> Hold<'a> {
    pub const fn new(f: Action) -> Hold {
        Hold(f)
    }
}

impl<'a> HandleSwitchEvent<'a> for Hold<'a> {
    fn handle(&self, switch_event: &SwitchEvent) -> Option<Action<'a>> {
        match switch_event {
            SwitchEvent::Pressing(_) | SwitchEvent::Pressed(_) => Some(self.0),
            _ => None,
        }
    }
}

impl<'a> Tap<'a> {
    pub const fn new(f: Action) -> Tap {
        Tap(f)
    }
}

impl<'a> HandleSwitchEvent<'a> for Tap<'a> {
    fn handle(&self, switch_event: &SwitchEvent) -> Option<Action<'a>> {
        match switch_event {
            SwitchEvent::Pressing(_) => Some(self.0),
            _ => None,
        }
    }
}

impl<'a> OnOff<'a> {
    pub const fn new(f0: Action<'a>, f1: Action<'a>) -> OnOff<'a> {
        OnOff(f0, f1)
    }
}

impl<'a> HandleSwitchEvent<'a> for OnOff<'a> {
    fn handle(&self, switch_event: &SwitchEvent) -> Option<Action<'a>> {
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
    ($($x: tt),* $(,)?) => {
        $crate::switch_handler::Hold::new(
            $crate::action::Action::Report(
                &[$(rpt!($x)),*]
            )
        )
    };
}

// Keyboard Tap
#[macro_export]
macro_rules! kt {
    ($($x: tt),* $(,)?) => {
        $crate::switch_handler::Tap::new(
            $crate::action::Action::Report(
                &[$(rpt!($x)),*]
            )
        )
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
