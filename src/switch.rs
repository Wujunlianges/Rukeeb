use crate::switch_handler::SwitchHandle;

pub type Timestamp = u8;

pub trait Switch {
    fn update(&mut self, signal: bool) -> SwitchEvent;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwitchEvent {
    Released(Timestamp),
    Pressed(Timestamp),
    Pressing(Timestamp),
    Releasing(Timestamp),
}

impl Default for SwitchEvent {
    fn default() -> SwitchEvent {
        SwitchEvent::Released(0)
    }
}

impl SwitchEvent {
    pub fn new() -> SwitchEvent {
        Default::default()
    }
    pub fn is_release(&self) -> bool {
        matches!(*self, SwitchEvent::Released(_) | SwitchEvent::Releasing(_))
    }
    pub fn is_press(&self) -> bool {
        matches!(*self, SwitchEvent::Pressed(_) | SwitchEvent::Pressing(_))
    }
    pub fn is_releasing(&self) -> bool {
        matches!(*self, SwitchEvent::Releasing(_))
    }
    pub fn is_pressing(&self) -> bool {
        matches!(*self, SwitchEvent::Pressing(_))
    }
    pub fn is_released(&self) -> bool {
        matches!(*self, SwitchEvent::Released(_))
    }
    pub fn is_pressed(&self) -> bool {
        matches!(*self, SwitchEvent::Pressed(_))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwitchState {
    Released(Timestamp),
    Pressed(Timestamp),
}

impl SwitchState {
    pub fn press(&mut self) -> SwitchEvent {
        match *self {
            SwitchState::Pressed(i) => {
                *self = SwitchState::Pressed(i.saturating_add(1));
                SwitchEvent::Pressed(i.saturating_add(1))
            }
            SwitchState::Released(i) => {
                *self = SwitchState::Pressed(0);
                SwitchEvent::Pressing(i)
            }
        }
    }

    pub fn release(&mut self) -> SwitchEvent {
        match *self {
            SwitchState::Pressed(i) => {
                *self = SwitchState::Released(0);
                SwitchEvent::Releasing(i)
            }
            SwitchState::Released(i) => {
                *self = SwitchState::Released(i.saturating_add(1));
                SwitchEvent::Released(i.saturating_add(1))
            }
        }
    }

    pub fn proceed(&mut self) -> SwitchEvent {
        match *self {
            SwitchState::Pressed(i) => {
                *self = SwitchState::Pressed(i.saturating_add(1));
                SwitchEvent::Pressed(i.saturating_add(1))
            }
            SwitchState::Released(i) => {
                *self = SwitchState::Released(i.saturating_add(1));
                SwitchEvent::Released(i.saturating_add(1))
            }
        }
    }
}
