use crate::action::Action;
use crate::action_handler::HandleAction;
use crate::key::KeyEvent;
use crate::report::Report;
use crate::switch_handler::HandleSwitchEvent;

mod chord;
pub use chord::Chord;

pub trait HandleKeyEvent<'a, const N: usize>: Sync {
    fn handle(
        &self,
        key_events: &mut [Option<KeyEvent>; N],
        action_handler: &mut dyn HandleAction,
    ) -> Result<(), Report>;
}

pub struct Keymap<'a, const N: usize> {
    layers: &'a [&'a [&'a dyn HandleSwitchEvent; N]],
}

impl<'a, const N: usize> Keymap<'a, N> {
    pub const fn new(layers: &'a [&'a [&'a dyn HandleSwitchEvent; N]]) -> Keymap<'a, N> {
        Keymap { layers }
    }
}

impl<'a, const N: usize> HandleKeyEvent<'a, N> for Keymap<'a, N> {
    fn handle(
        &self,
        key_events: &mut [Option<KeyEvent>; N],
        action_handler: &mut dyn HandleAction,
    ) -> Result<(), Report> {
        key_events
            .iter_mut()
            .enumerate()
            .try_for_each(|(idx, key_event)| {
                if let Some((key_layer, switch_event)) = key_event {
                    if let Some(action) = self.layers[*key_layer][idx].handle(switch_event) {
                        action_handler.handle(action)?;
                    }
                    *key_event = None;
                }
                Ok(())
            })
    }
}

#[macro_export]
macro_rules! layer {
    ($($($x:expr),+ $(,)?);* $(;)?) => {
        $crate::layer!(@munch [] [$($($x,)*;)*])
    };
    (@munch [] [$($x0:expr, $($x:expr,)*;)*]) => {
        $crate::layer!(@munch [$($x0,)*] [$($($x,)*;)*])
    };
    (@munch [$($x0:expr,)*] [$($x1:expr, $($x:expr,)*;)*]) => {
        $crate::layer!(@munch [$($x0,)*$($x1,)*] [$($($x,)*;)*])
    };
    (@munch [$($x:expr,)*] [$(;)*]) => {
        $crate::layer!(@finish [$($x,)*])
    };
    (@finish [$($x:expr,)*]) => {
        [$(&$x,)*]
    };
}

#[macro_export]
macro_rules! layers {
    ($([$($($x:expr),+ $(,)?);* $(;)?]),* $(,)?) => {
        [$(&$crate::layer!($($($x),+;)*)),*]
    };
}

#[macro_export]
macro_rules! keymap {
    ([$([$($($x:expr),+ $(,)?);* $(;)?]),* $(,)?]) => {
        $crate::key_handler::Keymap::new(&$crate::layers![$([$($($x),+);*]),*])
    };
}
