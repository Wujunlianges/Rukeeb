use heapless::vec::Vec;

use crate::function::Function;
use crate::key::KeyEvent;
use crate::key_handler::KeyHandle;
use crate::switch::SwitchEvent;
use crate::switch_handler::SwitchHandle;

type KeyPair = (usize, usize, usize, &'static dyn SwitchHandle); // layer, key0_idx, key1_idx

pub struct Chord {
    key_pairs: &'static [KeyPair],
}

impl Chord {
    pub const fn new(key_pairs: &'static [KeyPair]) -> Chord {
        Chord { key_pairs }
    }
}

impl<const N: usize> KeyHandle<N> for Chord {
    fn handle(
        &self,
        key_events: &mut [Option<KeyEvent>; N],
        functions: &mut Vec<Function, N>,
    ) -> Result<(), Function> {
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
