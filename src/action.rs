use crate::report::Report;

#[derive(Clone, Copy)]
pub enum Action<'a> {
    Report(&'a [Report]), // todo: make it Report only.
    Layer(u8),
}

// Keyboard
#[macro_export]
macro_rules! k {
    ($($x: tt),* $(,)?) => {
        $crate::action::Action::Report(
            &[$($crate::rpt!($x)),*]
        )
    };
}

// Layer
#[macro_export]
macro_rules! l {
    ($x: tt) => {
        $crate::action::Action::Layer($x)
    };
}
