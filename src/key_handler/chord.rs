use heapless::vec::Vec;

use crate::function::Function;
use crate::key::KeyEvent;
use crate::key_handler::KeyHandle;
use crate::switch::SwitchEvent;
use crate::switch_handler::SwitchHandle;

type KeyPair<'a> = (usize, usize, usize, &'a dyn SwitchHandle<'a>); // layer, key0_idx, key1_idx

pub struct Chord<'a> {
    key_pairs: &'a [KeyPair<'a>],
}

impl<'a> Chord<'a> {
    pub const fn new(key_pairs: &'a [KeyPair<'a>]) -> Chord<'a> {
        Chord { key_pairs }
    }
}

impl<'a, const N: usize> KeyHandle<'a, N> for Chord<'a> {
    fn handle(
        &self,
        key_events: &mut [Option<KeyEvent>; N],
        functions: &mut Vec<Function<'a>, N>,
    ) -> Result<(), Function<'a>> {
        self.key_pairs
            .iter()
            .try_for_each(|(layer, key0_idx, key1_idx, handler)| {
                match (key_events[*key0_idx], key_events[*key1_idx]) {
                    (Some((key_layer0, switch_event0)), Some((key_layer1, switch_event1))) => {
                        if *layer == key_layer0 && *layer == key_layer1 {
                            if let Some(function) = handler.handle(&switch_event0) {
                                functions.push(function)?;
                            }
                            if let Some(function) = handler.handle(&switch_event1) {
                                functions.push(function)?;
                            }
                            key_events[*key0_idx] = None;
                            key_events[*key1_idx] = None;
                        }
                        Ok(())
                    }
                    _ => Ok(()),
                }
            })
    }
}

#[macro_export]
macro_rules! chrd {
    ([$(($l: literal, $k0: literal, $k1: literal, $handler: expr)),+ $(,)?]) => {
        $crate::key_handler::Chord::new(&[$(($l, $k0, $k1, &$handler)),+])
    };
}
