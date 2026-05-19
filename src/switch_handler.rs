use crate::function::Function;
use crate::switch::SwitchEvent;

pub mod holdtap;

pub trait SwitchHandle: Sync {
    fn handle(&self, switch_event: &SwitchEvent) -> Option<Function>;
}

pub struct Hold(Function);
pub struct Tap(Function);
pub struct OnOff(Function, Function);

impl Hold {
    pub const fn new(f: Function) -> Hold {
        Hold(f)
    }
}

impl SwitchHandle for Hold {
    fn handle(&self, switch_event: &SwitchEvent) -> Option<Function> {
        match switch_event {
            SwitchEvent::Pressing(_) | SwitchEvent::Pressed(_) => Some(self.0),
            _ => None,
        }
    }
}

impl Tap {
    pub const fn new(f: Function) -> Tap {
        Tap(f)
    }
}

impl SwitchHandle for Tap {
    fn handle(&self, switch_event: &SwitchEvent) -> Option<Function> {
        match switch_event {
            SwitchEvent::Pressing(_) => Some(self.0),
            _ => None,
        }
    }
}

impl OnOff {
    pub const fn new(f0: Function, f1: Function) -> OnOff {
        OnOff(f0, f1)
    }
}

impl SwitchHandle for OnOff {
    fn handle(&self, switch_event: &SwitchEvent) -> Option<Function> {
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
