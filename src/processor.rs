use crate::event::Event;
use crate::function::Function;
use crate::handler::{self, Handle};

pub mod chord;

pub trait Process<const N: usize, const L: usize>: Sync {
    fn process(
        &self,
        handlers: &mut [Option<&'static dyn Handle>; N],
        events: &[Event; N],
        layer: usize,
    );
}

pub struct Processor<const N: usize, const L: usize> {
    keys: [[&'static dyn Handle; N]; L],
}

impl<const N: usize, const L: usize> Processor<N, L> {
    pub const fn new(keys: [[&'static dyn Handle; N]; L]) -> Processor<N, L> {
        Processor { keys }
    }
}

impl<const N: usize, const L: usize> Process<N, L> for Processor<N, L> {
    fn process(
        &self,
        handlers: &mut [Option<&'static dyn Handle>; N],
        events: &[Event; N],
        layer: usize,
    ) {
        let keys = &self.keys[layer];

        handlers
            .iter_mut()
            .zip(events.iter().zip(keys.iter()))
            .for_each(|(handler, (event, key))| {
                if matches!(event, Event::Press(_)) && handler.is_none() {
                    *handler = Some(*key);
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
        $crate::keys!(@key [$([$($x,)*],)*])
    };
    (@key [$([$($x:expr,)*],)*]) => {
        [$([$(&$x,)*],)*]
    };
}
