#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Pressed(u8),
    Pressing(u8),
    Releasing(u8),
    Released(u8),
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
