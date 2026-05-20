use crate::switch::{Switch, SwitchEvent, SwitchState, Tick};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Debouncer<const DT: Tick> {
    signal: bool,
    duration: Tick,
    state: SwitchState,
}

impl<const DT: Tick> Debouncer<DT> {
    pub fn new() -> Debouncer<DT> {
        Debouncer {
            signal: false,
            duration: 0,
            state: SwitchState::Released(0),
        }
    }

    pub fn reset(&mut self) {
        self.signal = false;
        self.duration = 0;
    }
}

impl<const DT: Tick> Switch for Debouncer<DT> {
    fn update(&mut self, signal: bool) -> SwitchEvent {
        self.duration = match self.signal == signal {
            true => self.duration.saturating_add(1),
            false => 0,
        };
        self.signal = signal;
        match (self.duration >= DT, self.signal) {
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
            assert_eq!(debouncer.update(false), SwitchEvent::Released(i));
        });

        (1..=5).for_each(|i| {
            assert_eq!(debouncer.update(true), SwitchEvent::Released(10 + i));
        });

        assert_eq!(debouncer.update(true), SwitchEvent::Pressing(15));

        (1..=5).for_each(|i| {
            assert_eq!(debouncer.update(false), SwitchEvent::Pressed(i));
        });

        assert_eq!(debouncer.update(false), SwitchEvent::Releasing(5));
    }

    #[test]
    fn wobble() {
        let mut debouncer = Debouncer::<5>::new();

        (1..=10).for_each(|i| {
            debouncer.update(false);
            debouncer.update(true);
            debouncer.update(true);
            debouncer.update(true);
            debouncer.update(true);
            assert_eq!(debouncer.update(true), SwitchEvent::Released(6 * i));
        });
    }

    #[test]
    fn no_debounce() {
        let mut debouncer = Debouncer::<0>::new();

        assert_eq!(debouncer.update(true), SwitchEvent::Pressing(0));
        (1..=10).for_each(|i| {
            assert_eq!(debouncer.update(true), SwitchEvent::Pressed(i));
        });
    }
}
