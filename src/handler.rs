use crate::action::{Act, Action};
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

// From https://veykril.github.io/tlborm/decl-macros/building-blocks/counting.html#bit-twiddling
macro_rules! count_tts {
    () => { 0 };
    ($odd:tt $($a:tt $b:tt)*) => { ($crate::handler::count_tts!($($a)*) << 1) | 1 };
    ($($a:tt $even:tt)*) => { $crate::handler::count_tts!($($a)*) << 1 };
}
pub(crate) use count_tts;

macro_rules! count_layers {
    ($([$($($x:expr),*);*]),*) => {$crate::handler::count_tts!($([$($($x),*);*])*)};
}
pub(crate) use count_layers;

macro_rules! count_keys {
    ([$($($x0:expr),*);*], $([$($($x:expr),*);*]),*) => {$crate::handler::count_tts!($($($x0)*)*)};
}
pub(crate) use count_keys;

macro_rules! keys {
    ($([$($($x:expr),+ $(,)?);* $(;)?]),* $(,)?) => {
        $crate::handler::keys!(@layer [] $([$($($x,)*;)*],)*)
    };
    (@layer [] $([$($x0:expr, $($x:expr,)*;)*],)*) => {
        $crate::handler::keys!(@layer [$([$($x0,)*],)*] $([$($($x,)*;)*],)*)
    };
    (@layer [$([$($x0:expr,)*],)*] $([$($x1:expr, $($x:expr,)*;)*],)*) => {
        $crate::handler::keys!(@layer [$([$($x0,)*$($x1,)*],)*] $([$($($x,)*;)*],)*)
    };
    (@layer [$([$($x:expr,)*],)*] $([$(;)*],)*) => {
        $crate::handler::keys!(@key [] $([$($x,)*],)*)
    };
    (@key [] $([$x0:expr, $($x:expr,)*],)*) => {
        $crate::handler::keys!(@key [[$($x0,)*],] $([$($x,)*],)*)
    };
    (@key [$([$($k:expr,)*],)*] $([$x0:expr, $($x:expr,)*],)*) => {
        $crate::handler::keys!(@key [$([$($k,)*],)* [$($x0,)*],] $([$($x,)*],)*)
    };
    (@key [$([$($k:expr,)*],)*] $([],)*) => {
        [$([$($k,)*],)*]
    };
}
pub(crate) use keys;

#[macro_export]
macro_rules! key_handler {
    ($([$($($x:expr),+ $(,)?);* $(;)?]),* $(,)?) => {
        {
            const N_LAYERS: usize = $crate::handler::count_layers!($([$($($x),*);*]),*);
            const N_KEYS: usize = $crate::handler::count_keys!($([$($($x),*);*]),*);
            const KEYS: [[&'static dyn $crate::action::Act; N_LAYERS] ;N_KEYS] = $crate::handler::keys!($([$($(&$x),*);*]),*);
            $crate::handler::KeyHandler::new(&KEYS)
        }
    };
}
