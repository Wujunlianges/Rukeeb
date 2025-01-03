#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Pressed(usize),
    Press(usize),
    Release(usize),
    Released(usize),
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
