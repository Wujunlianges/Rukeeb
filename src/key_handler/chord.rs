use crate::action_handler::HandleAction;
use crate::key::KeyEvent;
use crate::key_handler::HandleKeyEvent;
use crate::report::Report;
use crate::switch_handler::HandleSwitchEvent;

type ChordHandler<'a> = (usize, usize, usize, &'a dyn HandleSwitchEvent); // layer, key0_idx, key1_idx, switch_handler

pub struct Chord<'a> {
    chord_handlers: &'a [ChordHandler<'a>],
}

impl<'a> Chord<'a> {
    pub const fn new(chord_handlers: &'a [ChordHandler<'a>]) -> Chord<'a> {
        Chord { chord_handlers }
    }
}

impl<'a, const N: usize> HandleKeyEvent<'a, N> for Chord<'a> {
    fn handle(
        &self,
        key_events: &mut [Option<KeyEvent>; N],
        action_handler: &mut dyn HandleAction,
    ) -> Result<(), Report> {
        self.chord_handlers.iter().try_for_each(
            |(layer, key0_idx, key1_idx, switch_handler)| match (
                key_events[*key0_idx],
                key_events[*key1_idx],
            ) {
                (Some((key_layer0, switch_event0)), Some((key_layer1, switch_event1))) => {
                    if *layer == key_layer0 && *layer == key_layer1 {
                        if let Some(action) = switch_handler.handle(&switch_event0) {
                            action_handler.handle(action)?;
                        }
                        if let Some(action) = switch_handler.handle(&switch_event1) {
                            action_handler.handle(action)?;
                        }
                        key_events[*key0_idx] = None;
                        key_events[*key1_idx] = None;
                    }
                    Ok(())
                }
                _ => Ok(()),
            },
        )
    }
}

#[macro_export]
macro_rules! chrd {
    ([$(($l: literal, $k0: literal, $k1: literal, $switch_handler: expr)),+ $(,)?]) => {
        $crate::key_handler::Chord::new(&[$(($l, $k0, $k1, &$switch_handler)),+])
    };
}
