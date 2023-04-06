use crate::action::{Act, Action};
use crate::event::Event;
use crate::handler::Handle;
use crate::performer::Performer;

pub struct Comb<const L: usize> {
    pub id: usize,
    pub actions: [Option<&'static [Action]>; L],
}

impl<const L: usize> Comb<L> {
    pub const fn new(id: usize, actions: [Option<&'static [Action]>; L]) -> Comb<L> {
        Comb { id, actions }
    }
}

impl<const L: usize> Handle for Comb<L> {
    fn handle(&self, _: usize, events: &mut [Event], performer: &mut Performer) {
        if let Some(actions) = self.actions[performer.current_layer()] {
            if let Event::Press(_) = events[self.id] {
                for action in actions {
                    performer.perform(self.id, action);
                }
            }
        }
        events[self.id] = Event::Released(0);
    }
}

#[macro_export]
macro_rules! cmb {
    ($i: literal, [$($x:expr),* $(,)?]) => {
        $crate::handler::comb::Comb::new($i, [$($x),*])
    };
}
