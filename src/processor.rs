use crate::event::Event;
use crate::handler::Handle;

pub mod chord;

pub trait Process<const N: usize>: Sync {
    fn process(&self, handlers: &mut [Option<&'static dyn Handle>; N], events: &[Event; N]);
}

pub struct Processor<const N: usize> {
    keys: [&'static dyn Handle; N],
}

impl<const N: usize> Processor<N> {
    pub const fn new(keys: [&'static dyn Handle; N]) -> Processor<N> {
        Processor { keys }
    }
}

impl<const N: usize> Process<N> for Processor<N> {
    fn process(&self, handlers: &mut [Option<&'static dyn Handle>; N], events: &[Event; N]) {
        handlers
            .iter_mut()
            .zip(events.iter().zip(self.keys.iter()))
            .for_each(|(handler, (event, key))| {
                if matches!(event, Event::Pressing(_)) && handler.is_none() {
                    *handler = Some(*key);
                }
            });
    }
}

#[macro_export]
macro_rules! keys {
    ($($($x:expr),+ $(,)?);* $(;)?) => {
        $crate::keys!(@layer [] [$($($x,)*;)*])
    };
    (@layer [] [$($x0:expr, $($x:expr,)*;)*]) => {
        $crate::keys!(@layer [$($x0,)*] [$($($x,)*;)*])
    };
    (@layer [$($x0:expr,)*] [$($x1:expr, $($x:expr,)*;)*]) => {
        $crate::keys!(@layer [$($x0,)*$($x1,)*] [$($($x,)*;)*])
    };
    (@layer [$($x:expr,)*] [$(;)*]) => {
        $crate::keys!(@key [$($x,)*])
    };
    (@key [$($x:expr,)*]) => {
        [$(&$x,)*]
    };
}

#[macro_export]
macro_rules! processor {
    ([$($($x:expr),+ $(,)?);* $(;)?]) => {
        $crate::processor::Processor::new($crate::keys![$($($x,)*;)*])
    };
}
