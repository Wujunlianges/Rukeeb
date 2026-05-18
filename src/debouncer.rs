use crate::event::{Event, State, Timestamp};

pub trait Debounce {
    fn debounce(&mut self, switch: bool) -> Event;
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Debouncer<const DT: Timestamp> {
    switch: bool,
    duration: Timestamp,
    state: State,
}

impl<const DT: Timestamp> Debouncer<DT> {
    pub fn new() -> Debouncer<DT> {
        Debouncer {
            ..Default::default()
        }
    }
}

impl<const DT: Timestamp> Debounce for Debouncer<DT> {
    fn debounce(&mut self, switch: bool) -> Event {
        self.duration = match self.switch == switch {
            true => self.duration.saturating_add(1),
            false => 0,
        };
        self.switch = switch;
        match (self.duration >= DT, self.switch) {
            (true, true) => self.state.press(),
            (true, false) => self.state.release(),
            (false, _) => self.state.proceed(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hold() {
        let mut debouncer = Debouncer::<5>::new();

        (1..=10).for_each(|i| {
            assert_eq!(debouncer.debounce(false), Event::Released(i));
        });

        (1..=5).for_each(|i| {
            assert_eq!(debouncer.debounce(true), Event::Released(10 + i));
        });

        assert_eq!(debouncer.debounce(true), Event::Pressing(15));

        (1..=5).for_each(|i| {
            assert_eq!(debouncer.debounce(false), Event::Pressed(i));
        });

        assert_eq!(debouncer.debounce(false), Event::Releasing(5));
    }

    #[test]
    fn wobble() {
        let mut debouncer = Debouncer::<5>::new();

        (1..=10).for_each(|i| {
            debouncer.debounce(false);
            debouncer.debounce(true);
            debouncer.debounce(true);
            debouncer.debounce(true);
            debouncer.debounce(true);
            assert_eq!(debouncer.debounce(true), Event::Released(6 * i));
        });
    }

    #[test]
    fn no_debounce() {
        let mut debouncer = Debouncer::<0>::new();

        assert_eq!(debouncer.debounce(true), Event::Pressing(0));
        (1..=10).for_each(|i| {
            assert_eq!(debouncer.debounce(true), Event::Pressed(i));
        });
    }
}
