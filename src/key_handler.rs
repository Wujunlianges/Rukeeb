use heapless::Vec;

use crate::function::Function;
use crate::key::KeyEvent;
use crate::switch_handler::SwitchHandle;

mod chord;
pub use chord::Chord;

pub trait KeyHandle<const N: usize>: Sync {
    fn handle(
        &self,
        key_events: &mut [Option<KeyEvent>; N],
        functions: &mut Vec<Function, N>,
    ) -> Result<(), Function>;
}

pub struct Keymap<const N: usize> {
    layers: &'static [&'static [&'static dyn SwitchHandle; N]],
}

impl<const N: usize> Keymap<N> {
    pub const fn new(layers: &'static [&'static [&'static dyn SwitchHandle; N]]) -> Keymap<N> {
        Keymap { layers }
    }
}

impl<const N: usize> KeyHandle<N> for Keymap<N> {
    fn handle(
        &self,
        key_events: &mut [Option<KeyEvent>; N],
        functions: &mut Vec<Function, N>,
    ) -> Result<(), Function> {
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
