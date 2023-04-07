use crate::event::Event;

#[derive(Clone, Copy, Default)]
pub struct State(pub bool, pub usize, pub Event);

impl State {
    pub fn new() -> State {
        Default::default()
    }

    pub fn disable(&mut self) {
        self.0 = false;
    }

    pub fn update(&mut self, event: Event, layer: usize) {
        self.0 = true;
        self.2 = event;
        if let Event::Press(_) = self.2 {
            self.1 = layer;
        }
    }
}
