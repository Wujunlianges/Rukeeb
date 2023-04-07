use crate::action::Action;
use crate::event::Event;
use crate::handler::{Handle, State};
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
    fn handle(&self, states: &mut [State; N], performer: &mut Performer<L>) {
        let State(enabled, layer, event) = &mut states[self.id];
        if *enabled {
            if let Some(actions) = self.actions[*layer] {
                if let Event::Press(_) = event {
                    actions.iter().for_each(|action| {
                        performer.perform(action);
                    });
                }
            }
            *enabled = false;
        }
    }
}

#[macro_export]
macro_rules! cmb {
    ($i: literal, [$($x:expr),* $(,)?]) => {
        $crate::handler::comb::Comb::new($i, [$($x),*])
    };
}
