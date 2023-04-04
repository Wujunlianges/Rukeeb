use core::sync::atomic::{AtomicBool, Ordering};

use crate::action::Action;
use crate::event::Event;
use crate::performer::Performer;

pub trait Handle: Sync {
    fn handle(&self, id: usize, events: &mut [Event], performer: &mut Performer);
}

pub struct Chord<const L: usize> {
    is_triggered: AtomicBool,
    ids: (usize, usize),
    actions: [Option<Action>; L],
}

impl<const L: usize> Chord<L> {
    pub const fn new(ids: (usize, usize), actions: [Option<Action>; L]) -> Chord<L> {
        Chord {
            is_triggered: AtomicBool::new(false),
            ids,
            actions,
        }
    }
}

impl<const L: usize> Handle for Chord<L> {
    fn handle(&self, id: usize, events: &mut [Event], performer: &mut Performer) {
        let (id0, id1) = self.ids;

        if let Some(action) = &self.actions[performer.current_layer()] {
            match self.is_triggered.load(Ordering::Relaxed) {
                false => match (events[id0], events[id1]) {
                    (Event::Press(_), Event::Pressed(_))
                    | (Event::Pressed(_), Event::Press(_))
                    | (Event::Press(_), Event::Press(_)) => {
                        self.is_triggered.store(true, Ordering::Relaxed);
                        performer.perform(id, action);
                        events[id0] = Event::Released(0);
                        events[id1] = Event::Released(0);
                    }
                    _ => {}
                },
                true => {
                    match (events[id0], events[id1]) {
                        (Event::Release(_), Event::Released(_))
                        | (Event::Released(_), Event::Release(_))
                        | (Event::Release(_), Event::Release(_)) => {
                            self.is_triggered.store(false, Ordering::Relaxed);
                        }
                        _ => {}
                    }
                    events[id0] = Event::Released(0);
                    events[id1] = Event::Released(0);
                }
            }
        }
    }
}

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
macro_rules! chrd {
    ($i1:literal, $i2:literal, [$($x:expr),* $(,)?]) => {
        $crate::handler::Chord::new(($i1, $i2), [$($x),*])
    };
}

#[macro_export]
macro_rules! comb {
    ($i: literal, [$($x:expr),* $(,)?]) => {
        $crate::handler::Comb::new($i, [$($x),*])
    };
}

#[macro_export]
macro_rules! handlers {
    ($($x:expr),* $(,)?) => {
        [$(&$x),*]
    };
}

#[cfg(test)]
#[no_implicit_prelude]
mod test {
    use ::core::option::Option::{None, Some};
    use ::core::sync::atomic::AtomicBool;

    use crate::action::Action;
    use crate::handler::{Chord, Comb, Handle};
    use crate::{handlers, kb};

    static CHORD1: Chord<2> = chrd!(0, 2, [Some(kb!(Q)), None]);
    static COMB1: Comb<2> = comb!(2, [None, Some(&[kb!(C), kb!(D)])]);

    #[test]
    fn test_handler_macros() {
        let _: [&'static dyn Handle; 2] = handlers![CHORD1, COMB1];
    }
}
