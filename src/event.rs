#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Event {
    Pressed(usize),
    Press(usize),
    Release(usize),
    Released(usize),
}

impl Default for Event {
    fn default() -> Self {
        Event::Released(0)
    }
}

impl Event {
    pub fn new() -> Event {
        Default::default()
    }
}
