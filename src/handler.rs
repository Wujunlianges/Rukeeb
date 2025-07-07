use crate::event::Event;
use crate::function::Function;

pub mod holdtap;

pub trait Handle: Sync {
    fn handle(&self, event: &Event) -> Option<Function>;
}

pub struct Hold(Function);
pub struct Tap(Function);
pub struct OnOff(Function, Function);

impl Hold {
    pub const fn new(f: Function) -> Hold {
        Hold(f)
    }
}

impl Handle for Hold {
    fn handle(&self, event: &Event) -> Option<Function> {
        match event {
            Event::Pressing(_) | Event::Pressed(_) => Some(self.0),
            _ => None,
        }
    }
}

impl Tap {
    pub const fn new(f: Function) -> Tap {
        Tap(f)
    }
}

impl Handle for Tap {
    fn handle(&self, event: &Event) -> Option<Function> {
        match event {
            Event::Pressing(_) => Some(self.0),
            _ => None,
        }
    }
}

impl OnOff {
    pub const fn new(f0: Function, f1: Function) -> OnOff {
        OnOff(f0, f1)
    }
}

impl Handle for OnOff {
    fn handle(&self, event: &Event) -> Option<Function> {
        match event {
            Event::Pressing(_) => Some(self.0),
            Event::Releasing(_) => Some(self.1),
            _ => None,
        }
    }
}

// Keyboard Hold
#[macro_export]
macro_rules! kh {
    ($($x: tt),* $(,)?) => {
        $crate::handler::Hold::new(
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
        $crate::handler::Tap::new(
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
        $crate::handler::Tap::new(
            $crate::function::Function::Layer($x)
        )
    };
}

// Layer OnOff
#[macro_export]
macro_rules! lo {
    ($x0:tt, $x1:tt) => {
        $crate::handler::OnOff::new(
            $crate::function::Function::Layer($x0),
            $crate::function::Function::Layer($x1)
        )
    };
}
