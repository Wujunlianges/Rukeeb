use crate::action::Act;
use crate::event::Event;
use crate::performer::Performer;

pub mod chord;
pub mod comb;

pub trait Handle<const N: usize, const L: usize>: Sync {
    fn handle(
        &self,
        layers: &[usize; N],
        events: &[Event; N],
        enabled: &mut [bool; N],
        performer: &mut Performer<L>,
    );
}

pub struct KeyHandler<const N: usize, const L: usize> {
    keys: &'static [[&'static dyn Act; L]; N],
}

impl<const N: usize, const L: usize> KeyHandler<N, L> {
    pub const fn new(keys: &'static [[&'static dyn Act; L]; N]) -> KeyHandler<N, L> {
        KeyHandler { keys }
    }
}

impl<const N: usize, const L: usize> Handle<N, L> for KeyHandler<N, L> {
    fn handle(
        &self,
        layers: &[usize; N],
        events: &[Event; N],
        enabled: &mut [bool; N],
        performer: &mut Performer<L>,
    ) {
        self.keys
            .iter()
            .zip(layers.iter())
            .zip(events.iter())
            .zip(enabled.iter())
            .for_each(|(((key, layer), event), e)| {
                if *e {
                    if let Some(action) = key[*layer].act(event) {
                        performer.perform(action)
                    }
                }
            });
    }
}

#[macro_export]
macro_rules! keys {
    ($([$($($x:expr),+ $(,)?);* $(;)?]),* $(,)?) => {
        $crate::keys!(@layer [] $([$($($x,)*;)*],)*)
    };
    (@layer [] $([$($x0:expr, $($x:expr,)*;)*],)*) => {
        $crate::keys!(@layer [$([$($x0,)*],)*] $([$($($x,)*;)*],)*)
    };
    (@layer [$([$($x0:expr,)*],)*] $([$($x1:expr, $($x:expr,)*;)*],)*) => {
        $crate::keys!(@layer [$([$($x0,)*$($x1,)*],)*] $([$($($x,)*;)*],)*)
    };
    (@layer [$([$($x:expr,)*],)*] $([$(;)*],)*) => {
        $crate::keys!(@key [] $([$($x,)*],)*)
    };
    (@key [] $([$x0:expr, $($x:expr,)*],)*) => {
        $crate::keys!(@key [[$($x0,)*],] $([$($x,)*],)*)
    };
    (@key [$([$($k:expr,)*],)*] $([$x0:expr, $($x:expr,)*],)*) => {
        $crate::keys!(@key [$([$($k,)*],)* [$($x0,)*],] $([$($x,)*],)*)
    };
    (@key [$([$($k:expr,)*],)*] $([],)*) => {
        [$([$(&$k,)*],)*]
    };
}
