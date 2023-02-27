use crate::event::Event;
use crate::hid_report::HIDReport;

pub trait Act: Sync {
    fn event(&self, event: &Event) -> Option<&KeyboardAction>;
}

pub struct Action(pub &'static dyn Act);

impl Action {
    pub fn event(&self, event: &Event) -> Option<&KeyboardAction> {
        self.0.event(event)
    }
}

pub enum KeyboardAction {
    HIDReport(HIDReport),
    LayerAction(LayerAction),
}

pub enum LayerAction {
    DefaultLayer(usize),
    CurrentLayer(usize),
    UndoCurrentLayer(usize),
}

pub struct PressAction(pub KeyboardAction);
pub struct PressedAction(pub KeyboardAction);
pub struct ReleaseAction(pub KeyboardAction);
pub struct ReleasedAction(pub KeyboardAction);

impl Act for PressAction {
    fn event(&self, event: &Event) -> Option<&KeyboardAction> {
        match event {
            Event::Press(_) => Some(&self.0),
            _ => None,
        }
    }
}

impl Act for PressedAction {
    fn event(&self, event: &Event) -> Option<&KeyboardAction> {
        match event {
            Event::Press(_) | Event::Pressed(_) => Some(&self.0),
            _ => None,
        }
    }
}

impl Act for ReleaseAction {
    fn event(&self, event: &Event) -> Option<&KeyboardAction> {
        match event {
            Event::Release(_) => Some(&self.0),
            _ => None,
        }
    }
}

impl Act for ReleasedAction {
    fn event(&self, event: &Event) -> Option<&KeyboardAction> {
        match event {
            Event::Release(_) | Event::Released(_) => Some(&self.0),
            _ => None,
        }
    }
}

// Keyboard Action Macros

#[macro_export]
macro_rules! k {
    ($x: tt) => {
        KeyboardAction::HIDReport(HIDReport::Keyboard(Keyboard::$x))
    };
}

#[macro_export]
macro_rules! c {
    ($x: tt) => {
        KeyboardAction::HIDReport(HIDReport::Consumer(Consumer::$x))
    };
}

#[macro_export]
macro_rules! ld {
    ($x: tt) => {
        KeyboardAction::LayerAction(LayerAction::DefaultLayer($x))
    };
}

#[macro_export]
macro_rules! lc {
    ($x: tt) => {
        KeyboardAction::LayerAction(LayerAction::CurrentLayer($x))
    };
}

#[macro_export]
macro_rules! lu {
    ($x: tt) => {
        KeyboardAction::LayerAction(LayerAction::UndoCurrentLayer($x))
    };
}

// Action Macros

#[macro_export]
macro_rules! pk {
    ($x:tt) => {
        Action(&PressAction(k!($x)))
    };
}

#[macro_export]
macro_rules! pdk {
    ($x:tt) => {
        Action(&PressedAction(k!($x)))
    };
}

#[macro_export]
macro_rules! rk {
    ($x:tt) => {
        Action(&ReleaseAction(k!($x)))
    };
}

#[macro_export]
macro_rules! rdk {
    ($x:tt) => {
        Action(&ReleasedAction(k!($x)))
    };
}

#[macro_export]
macro_rules! pc {
    ($x:tt) => {
        Action(&PressAction(c!($x)))
    };
}

#[macro_export]
macro_rules! pdc {
    ($x:tt) => {
        Action(&PressedAction(c!($x)))
    };
}

#[macro_export]
macro_rules! rc {
    ($x:tt) => {
        Action(&ReleaseAction(c!($x)))
    };
}

#[macro_export]
macro_rules! rdc {
    ($x:tt) => {
        Action(&ReleasedAction(c!($x)))
    };
}

#[macro_export]
macro_rules! pld {
    ($x:tt) => {
        Action(&PressAction(ld!($x)))
    };
}

#[macro_export]
macro_rules! plc {
    ($x:tt) => {
        Action(&PressAction(lc!($x)))
    };
}

#[macro_export]
macro_rules! rlu {
    ($x:tt) => {
        Action(&ReleaseAction(lu!($x)))
    };
}
