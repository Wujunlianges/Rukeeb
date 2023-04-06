#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Press(usize),
    Release(usize),
    Pressed(usize),
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
