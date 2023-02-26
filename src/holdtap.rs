use crate::action::{Act, KeyboardAction};
use crate::event::Event;

pub struct HoldTap {
    pub thold: usize,
    pub hold: KeyboardAction,
    pub tap: KeyboardAction,
}

impl Act for HoldTap {
    fn event(&self, event: &Event) -> Option<&KeyboardAction> {
        match event {
            Event::Pressed(i) if *i == self.thold => Some(&self.hold),
            Event::Release(i) if *i < self.thold => Some(&self.tap),
            _ => None,
        }
    }
}

#[macro_export]
macro_rules! ht {
    ($thold:literal, $hold:tt, $tap: tt) => {
        Action(&HoldTap {
            thold: $thold,
            hold: kbc!($hold),
            tap: kbc!($tap),
        })
    };
}
