use crate::event::Event;
use crate::handler::{Function, Handle};

pub struct HoldTap {
    thold: u8,
    hold: Function,
    tap: Function,
}

impl HoldTap {
    pub const fn new(thold: u8, hold: Function, tap: Function) -> HoldTap {
        HoldTap { thold, hold, tap }
    }
}

impl Handle for HoldTap {
    fn handle(&self, event: &Event) -> Option<&Function> {
        match event {
            Event::Pressed(i) if *i == self.thold => Some(&self.hold),
            Event::Releasing(i) if *i < self.thold => Some(&self.tap),
            _ => None,
        }
    }
}

#[macro_export]
macro_rules! ht {
    ($thold:literal, $hold:expr, $tap: expr) => {
        $crate::handler::holdtap::HoldTap::new($thold, $hold, $tap)
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
