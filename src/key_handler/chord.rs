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
        self.chord_handlers
            .iter()
            .try_for_each(|(layer, key0_idx, key1_idx, switch_handler)| {
                if key_events[*key0_idx].is_some_and(|(key_layer, _)| key_layer == *layer)
                    && key_events[*key1_idx].is_some_and(|(key_layer, _)| key_layer == *layer)
                {
                    key_events[*key0_idx]
                        .take()
                        .map(|(_, switch_event)| {
                            switch_handler
                                .handle(&switch_event)
                                .map(|action| action_handler.handle(action))
                                .unwrap_or(Ok(()))
                        })
                        .unwrap_or(Ok(()))?;

                    key_events[*key1_idx]
                        .take()
                        .map(|(_, switch_event)| {
                            switch_handler
                                .handle(&switch_event)
                                .map(|action| action_handler.handle(action))
                                .unwrap_or(Ok(()))
                        })
                        .unwrap_or(Ok(()))?;
                }
                Ok(())
            })
    }
}

#[macro_export]
macro_rules! chord {
    ([$(($l: literal, $k0: literal, $k1: literal, $switch_handler: expr)),+ $(,)?]) => {
        $crate::key_handler::Chord::new(&[$(($l, $k0, $k1, &$switch_handler)),+])
    };
}
