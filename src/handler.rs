use crate::action::{Act, Action};
use crate::event::Event;
use crate::performer::Performer;

pub mod chord;
pub mod comb;

pub trait Handle: Sync {
    fn handle(&self, id: usize, events: &mut [Event], performer: &mut Performer);
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
    use crate::handler::chord::Chord;
    use crate::handler::comb::Comb;
    use crate::handler::Handle;
    use crate::{chrd, cmb, handler, kb};

    static CHORD1: Chord<2> = chrd!(0, 2, [Some(kb!(Q)), None]);
    static COMB1: Comb<2> = cmb!(2, [None, Some(&[kb!(C), kb!(D)])]);

    #[test]
    fn test_handler_macros() {
        let _: [&'static dyn Handle; 2] = handlers![CHORD1, COMB1];
    }
}
