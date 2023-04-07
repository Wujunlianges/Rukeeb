use crate::action::Action;
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

impl<const N: usize, const L: usize> Handle<N, L> for Comb<L> {
    fn handle(
        &self,
        layers: &[usize; N],
        events: &[Event; N],
        enabled: &mut [bool; N],
        performer: &mut Performer<L>,
    ) {
        if enabled[self.id] {
            if let Some(actions) = self.actions[layers[self.id]] {
                if let Event::Press(_) = events[self.id] {
                    for action in actions {
                        performer.perform(action);
                    }
                }
            }
            enabled[self.id] = false;
        }
    }
}

#[macro_export]
macro_rules! cmb {
    ($i: literal, [$($x:expr),* $(,)?]) => {
        $crate::handler::comb::Comb::new($i, [$($x),*])
    };
}
