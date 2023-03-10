use crate::action::{Act, KeyboardAction};
use crate::event::Event;

pub struct HoldTap {
    pub thold: usize,
    pub hold: KeyboardAction,
    pub tap: KeyboardAction,
}

impl Act for HoldTap {
    fn act(&self, event: &Event) -> Option<&KeyboardAction> {
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
        $crate::action::Action(&$crate::holdtap::HoldTap {
            thold: $thold,
            hold: $crate::kb!($hold),
            tap: $crate::kb!($tap),
        })
    };
}

#[cfg(test)]
#[no_implicit_prelude]
mod test {
    use crate::ht;

    #[test]
    fn test_ht_macros() {
        ht!(50, F, J);
    }
}
