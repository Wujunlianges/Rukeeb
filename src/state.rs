use crate::event::Event;

#[derive(Clone, Copy, Default)]
pub struct State(pub bool, pub Event, pub usize);

impl State {
    pub fn new() -> State {
        Default::default()
    }

    pub fn disable(&mut self) {
        self.0 = false;
    }

    pub fn enable(&mut self) {
        self.0 = true;
    }

    pub fn event(&mut self, event: Event) {
        self.1 = event;
    }

    pub fn layer(&mut self, layer: usize) {
        if let Event::Press(_) = self.1 {
            self.2 = layer;
        }
    }
}
