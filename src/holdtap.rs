use crate::action::{CustomAction, KeyboardAction};
use crate::event::Event;

#[derive(Clone, Copy)]
pub struct HoldTap {
    pub thold: usize,
    pub hold: KeyboardAction,
    pub tap: KeyboardAction,
}

impl CustomAction for HoldTap {
    fn event(&self, event: &Event) -> Option<KeyboardAction> {
        match event {
            Event::Pressed(i) if *i == self.thold => Some(self.hold),
            Event::Release(i) if *i < self.thold => Some(self.tap),
            _ => None,
        }
    }
}
