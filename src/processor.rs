use itertools::izip;

use crate::event::Event;
use crate::function::Function;
use crate::handler::Handle;

pub mod chord;

pub trait Process<const N: usize, const L: usize>: Sync {
    fn process(
        &self,
        handlers: &mut [Option<&'static dyn Handle>; N],
        events: &[Event; N],
        layer: usize,
    );
}

pub struct KeyProcessor<const N: usize, const L: usize> {
    keys: [[&'static dyn Handle; N]; L],
}

impl<const N: usize, const L: usize> KeyProcessor<N, L> {
    pub const fn new(keys: [[&'static dyn Handle; N]; L]) -> KeyProcessor<N, L> {
        KeyProcessor { keys }
    }
}

impl<const N: usize, const L: usize> Process<N, L> for KeyProcessor<N, L> {
    fn process(
        &self,
        handlers: &mut [Option<&'static dyn Handle>; N],
        events: &[Event; N],
        layer: usize,
    ) {
        let keys = &self.keys[layer];

        for (handler, event, key) in izip!(handlers, events, keys) {
            if let Event::Press(_) = event {
                if handler.is_none() {
                    *handler = Some(*key);
                }
            }
        }
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
