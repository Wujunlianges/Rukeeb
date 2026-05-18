pub type Timestamp = u8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Pressed(Timestamp),
    Pressing(Timestamp),
    Releasing(Timestamp),
    Released(Timestamp),
}

impl Default for Event {
    fn default() -> Event {
        Event::Released(0)
    }
}

impl Event {
    pub fn new() -> Event {
        Default::default()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Pressed(Timestamp),
    Released(Timestamp),
}

impl Default for State {
    fn default() -> State {
        State::Released(0)
    }
}

impl State {
    pub fn new() -> State {
        Default::default()
    }

    pub fn press(&mut self) -> Event {
        match *self {
            State::Pressed(i) => {
                *self = State::Pressed(i.saturating_add(1));
                Event::Pressed(i.saturating_add(1))
            }
            State::Released(i) => {
                *self = State::Pressed(0);
                Event::Pressing(i)
            }
        }
    }

    pub fn release(&mut self) -> Event {
        match *self {
            State::Pressed(i) => {
                *self = State::Released(0);
                Event::Releasing(i)
            }
            State::Released(i) => {
                *self = State::Released(i.saturating_add(1));
                Event::Released(i.saturating_add(1))
            }
        }
    }

    pub fn proceed(&mut self) -> Event {
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
