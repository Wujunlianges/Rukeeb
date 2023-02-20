#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Press(usize),
    Release(usize),
    Pressed(usize),
    Released(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Pressed(usize),
    Released(usize),
}

impl State {
    pub fn new() -> State {
        State::Released(1)
    }

    pub fn press(&mut self) -> Event {
        match *self {
            State::Pressed(i) => {
                *self = State::Pressed(i.saturating_add(1));
                Event::Pressed(i.saturating_add(1))
            }
            State::Released(i) => {
                *self = State::Pressed(0);
                Event::Press(i)
            }
        }
    }

    pub fn release(&mut self) -> Event {
        match *self {
            State::Pressed(i) => {
                *self = State::Released(0);
                Event::Release(i)
            }
            State::Released(i) => {
                *self = State::Released(i.saturating_add(1));
                Event::Released(i.saturating_add(1))
            }
        }
    }

    pub fn go_on(&mut self) -> Event {
        match *self {
            State::Pressed(i) => {
                *self = State::Pressed(i.saturating_add(1));
                Event::Pressed(i.saturating_add(1))
            }
            State::Released(i) => {
                *self = State::Released(i.saturating_add(1));
                Event::Released(i.saturating_add(1))
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Debouncer<const DT: usize> {
    buffer: State,
    state: State,
}

impl<const DT: usize> Debouncer<DT> {
    pub fn new() -> Debouncer<DT> {
        Debouncer {
            buffer: State::new(),
            state: State::new(),
        }
    }

    pub fn press(&mut self) -> Event {
        self.buffer.press();
        match self.buffer {
            State::Pressed(i) if i >= DT => self.state.press(),
            _ => self.state.go_on(),
        }
    }

    pub fn release(&mut self) -> Event {
        self.buffer.release();
        match self.buffer {
            State::Released(i) if i >= DT => self.state.release(),
            _ => self.state.go_on(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hold() {
        let mut debouncer = Debouncer::<5>::new();
        let mut event = Event::Released(1);

        for _ in 0..10 {
            event = debouncer.release();
        }
        assert_eq!(event, Event::Released(11));

        for _ in 0..11 {
            event = debouncer.press();
        }
        assert_eq!(event, Event::Pressed(5));

        for _ in 0..11 {
            event = debouncer.release();
        }
        assert_eq!(event, Event::Released(5));
    }

    #[test]
    fn wobble() {
        let mut debouncer = Debouncer::<5>::new();
        let mut event = Event::Released(1);

        for _ in 0..10 {
            debouncer.release();
            debouncer.press();
            debouncer.press();
            debouncer.press();
            debouncer.press();
            event = debouncer.press();
        }

        assert_eq!(event, Event::Released(61));
    }

    #[test]
    fn no_debounce() {
        let mut debouncer = Debouncer::<0>::new();
        let mut event = Event::Released(1);

        for _ in 0..10 {
            event = debouncer.press();
        }

        assert_eq!(event, Event::Pressed(9));
    }
}
