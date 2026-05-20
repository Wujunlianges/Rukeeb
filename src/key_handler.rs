use heapless::Vec;

use crate::function::Function;
use crate::key::KeyEvent;
use crate::switch_handler::SwitchHandle;

mod chord;
pub use chord::Chord;

pub trait KeyHandle<'a, const N: usize>: Sync {
    fn handle(
        &self,
        key_events: &mut [Option<KeyEvent>; N],
        functions: &mut Vec<Function<'a>, N>,
    ) -> Result<(), Function<'a>>;
}

pub struct Keymap<'a, const N: usize> {
    layers: &'a [&'a [&'a dyn SwitchHandle<'a>; N]],
}

impl<'a, const N: usize> Keymap<'a, N> {
    pub const fn new(layers: &'a [&'a [&'a dyn SwitchHandle<'a>; N]]) -> Keymap<'a, N> {
        Keymap { layers }
    }
}

impl<'a, const N: usize> KeyHandle<'a, N> for Keymap<'a, N> {
    fn handle(
        &self,
        key_events: &mut [Option<KeyEvent>; N],
        functions: &mut Vec<Function<'a>, N>,
    ) -> Result<(), Function<'a>> {
        key_events
            .iter_mut()
            .enumerate()
            .try_for_each(|(idx, key_event)| {
                if let Some((key_layer, switch_event)) = key_event {
                    if let Some(function) = self.layers[*key_layer][idx].handle(switch_event) {
                        functions.push(function)?;
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
