use crate::event::Event;

pub trait Debounce {
    fn debounce(&mut self, switch: bool) -> Event;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Counter {
    Pressed(u8),
    Released(u8),
}

impl Default for Counter {
    fn default() -> Counter {
        Counter::Released(0)
    }
}

impl Counter {
    pub fn new() -> Counter {
        Default::default()
    }

    pub fn press(&mut self) -> Event {
        match *self {
            Counter::Pressed(i) => {
                *self = Counter::Pressed(i.saturating_add(1));
                Event::Pressed(i.saturating_add(1))
            }
            Counter::Released(i) => {
                *self = Counter::Pressed(0);
                Event::Press(i)
            }
        }
    }

    pub fn release(&mut self) -> Event {
        match *self {
            Counter::Pressed(i) => {
                *self = Counter::Released(0);
                Event::Release(i)
            }
            Counter::Released(i) => {
                *self = Counter::Released(i.saturating_add(1));
                Event::Released(i.saturating_add(1))
            }
        }
    }

    pub fn remain(&mut self) -> Event {
        match *self {
            Counter::Pressed(i) => {
                *self = Counter::Pressed(i.saturating_add(1));
                Event::Pressed(i.saturating_add(1))
            }
            Counter::Released(i) => {
                *self = Counter::Released(i.saturating_add(1));
                Event::Released(i.saturating_add(1))
            }
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Debouncer<const DT: u8> {
    buffer: Counter,
    counter: Counter,
}

impl<const DT: u8> Debouncer<DT> {
    pub fn new() -> Debouncer<DT> {
        Debouncer {
            ..Default::default()
        }
    }

    pub fn press(&mut self) -> Event {
        self.buffer.press();
        match self.buffer {
            Counter::Pressed(i) if i >= DT => self.counter.press(),
            _ => self.counter.remain(),
        }
    }

    pub fn release(&mut self) -> Event {
        self.buffer.release();
        match self.buffer {
            Counter::Released(i) if i >= DT => self.counter.release(),
            _ => self.counter.remain(),
        }
    }
}

impl<const DT: u8> Debounce for Debouncer<DT> {
    fn debounce(&mut self, switch: bool) -> Event {
        match switch {
            true => self.press(),
            false => self.release(),
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
            assert_eq!(debouncer.release(), Event::Released(i));
        });

        (1..=5).for_each(|i| {
            assert_eq!(debouncer.press(), Event::Released(10 + i));
        });

        assert_eq!(debouncer.press(), Event::Press(15));

        (1..=5).for_each(|i| {
            assert_eq!(debouncer.release(), Event::Pressed(i));
        });

        assert_eq!(debouncer.release(), Event::Release(5));
    }

    #[test]
    fn wobble() {
        let mut debouncer = Debouncer::<5>::new();

        (1..=10).for_each(|i| {
            debouncer.release();
            debouncer.press();
            debouncer.press();
            debouncer.press();
            debouncer.press();
            assert_eq!(debouncer.press(), Event::Released(6 * i));
        });
    }

    #[test]
    fn no_debounce() {
        let mut debouncer = Debouncer::<0>::new();

        assert_eq!(debouncer.press(), Event::Press(0));
        (1..=10).for_each(|i| {
            assert_eq!(debouncer.press(), Event::Pressed(i));
        });
    }
}
