use crate::report::Report;

#[derive(Clone, Copy)]
pub enum Function<'a> {
    Report(&'a [Report]),
    Layer(u8),
}

// Keyboard
#[macro_export]
macro_rules! k {
    ($($x: tt),* $(,)?) => {
        $crate::function::Function::Report(
            &[$($crate::rpt!($x)),*]
        )
    };
}

// Layer
#[macro_export]
macro_rules! l {
    ($x: tt) => {
        $crate::function::Function::Layer($x)
    };
}
