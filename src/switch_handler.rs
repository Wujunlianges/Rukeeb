use crate::function::Function;
use crate::switch::SwitchEvent;

mod holdtap;
pub use holdtap::HoldTap;

pub trait SwitchHandle<'a>: Sync {
    fn handle(&self, switch_event: &SwitchEvent) -> Option<Function<'a>>;
}

pub struct Hold<'a>(Function<'a>);
pub struct Tap<'a>(Function<'a>);
pub struct OnOff<'a>(Function<'a>, Function<'a>);

impl<'a> Hold<'a> {
    pub const fn new(f: Function) -> Hold {
        Hold(f)
    }
}

impl<'a> SwitchHandle<'a> for Hold<'a> {
    fn handle(&self, switch_event: &SwitchEvent) -> Option<Function<'a>> {
        match switch_event {
            SwitchEvent::Pressing(_) | SwitchEvent::Pressed(_) => Some(self.0),
            _ => None,
        }
    }
}

impl<'a> Tap<'a> {
    pub const fn new(f: Function) -> Tap {
        Tap(f)
    }
}

impl<'a> SwitchHandle<'a> for Tap<'a> {
    fn handle(&self, switch_event: &SwitchEvent) -> Option<Function<'a>> {
        match switch_event {
            SwitchEvent::Pressing(_) => Some(self.0),
            _ => None,
        }
    }
}

impl<'a> OnOff<'a> {
    pub const fn new(f0: Function<'a>, f1: Function<'a>) -> OnOff<'a> {
        OnOff(f0, f1)
    }
}

impl<'a> SwitchHandle<'a> for OnOff<'a> {
    fn handle(&self, switch_event: &SwitchEvent) -> Option<Function<'a>> {
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
            $crate::function::Function::Report(
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
            $crate::function::Function::Report(
                &[$(rpt!($x)),*]
            )
        )
    };
}

// Layer Tap
#[macro_export]
macro_rules! lt {
    ($x:tt) => {
        $crate::switch_handler::Tap::new($crate::function::Function::Layer($x))
    };
}

// Layer OnOff
#[macro_export]
macro_rules! lo {
    ($x0:tt, $x1:tt) => {
        $crate::switch_handler::OnOff::new(
            $crate::function::Function::Layer($x0),
            $crate::function::Function::Layer($x1),
        )
    };
}
