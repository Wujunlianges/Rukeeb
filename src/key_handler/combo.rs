use crate::action_handler::HandleAction;
use crate::key::KeyEvent;
use crate::key_handler::HandleKeyEvent;
use crate::report::Report;
use crate::switch_handler::HandleSwitchEvent;

type ComboHandler<'a> = (usize, usize, &'a [&'a dyn HandleSwitchEvent]);

pub struct Combo<'a> {
    combo_handlers: &'a [&'a ComboHandler<'a>],
}

impl<'a> Combo<'a> {
    pub const fn new(combo_handlers: &'a [&'a ComboHandler<'a>]) -> Combo<'a> {
        Combo { combo_handlers }
    }
}

impl<'a, const N: usize> HandleKeyEvent<'a, N> for Combo<'a> {
    fn handle(
        &self,
        key_events: &mut [Option<KeyEvent>; N],
        action_handler: &mut dyn HandleAction,
    ) -> Result<(), Report> {
        self.combo_handlers
            .iter()
            .try_for_each(|(layer, idx, switch_handlers)| {
                key_events[*idx]
                    .take_if(|(key_layer, _)| *key_layer == *layer)
                    .map(|(_, switch_event)| {
                        switch_handlers.iter().try_for_each(|switch_handler| {
                            switch_handler
                                .handle(&switch_event)
                                .map(|action| action_handler.handle(action))
                                .unwrap_or(Ok(()))
                        })
                    })
                    .unwrap_or(Ok(()))
            })
    }
}

#[macro_export]
macro_rules! combo {
    ([$(($l: literal, $i: literal, [$($switch_handler: expr),+ $(,)?])),+ $(,)?]) => {
        $crate::key_handler::Combo::new(&[$(&($l, $i, &[$(&$switch_handler),+])),+])
    };
}
