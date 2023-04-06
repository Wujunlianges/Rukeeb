use crate::action::{Act, Action};
use crate::event::Event;

pub struct HoldTap {
    thold: usize,
    hold: Action,
    tap: Action,
}

impl HoldTap {
    pub const fn new(thold: usize, hold: Action, tap: Action) -> HoldTap {
        HoldTap { thold, hold, tap }
    }
}

impl Act for HoldTap {
    fn act(&self, event: &Event) -> Option<&Action> {
        match event {
            Event::Pressed(i) if *i == self.thold => Some(&self.hold),
            Event::Release(i) if *i < self.thold => Some(&self.tap),
            _ => None,
        }
    }
}

#[macro_export]
macro_rules! ht {
    ($thold:literal, $hold:expr, $tap: expr) => {
        $crate::action::holdtap::HoldTap::new($thold, $hold, $tap)
    };
}

#[cfg(test)]
#[no_implicit_prelude]
mod test {
    use crate::{ht, kb};

    #[test]
    fn test_ht_macros() {
        ht!(50, kb!(F), kb!(J));
    }
}
