use crate::switch::SwitchEvent;
use crate::switch_handler::SwitchHandle;

pub mod chord;

pub trait Process<const N: usize>: Sync {
    fn feed_handlers(
        &self,
        handlers: &mut [Option<&'static dyn SwitchHandle>; N],
        events: &[SwitchEvent; N],
    );
}

pub struct Processor<const N: usize> {
    keys: [&'static dyn SwitchHandle; N],
}

impl<const N: usize> Processor<N> {
    pub const fn new(keys: [&'static dyn SwitchHandle; N]) -> Processor<N> {
        Processor { keys }
    }
}

impl<const N: usize> Process<N> for Processor<N> {
    fn feed_handlers(
        &self,
        handlers: &mut [Option<&'static dyn SwitchHandle>; N],
        events: &[SwitchEvent; N],
    ) {
        handlers
            .iter_mut()
            .zip(events.iter().zip(self.keys.iter()))
            .for_each(|(handler, (switch_event, key))| {
                if matches!(switch_event, SwitchEvent::Pressing(_)) && handler.is_none() {
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
        $crate::key_handler::Processor::new($crate::keys![$($($x,)*;)*])
    };
}
