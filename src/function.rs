use crate::report::Report;

#[derive(Clone, Copy)]
pub enum Function {
    Report(Report),
    Layer(u8),
}

// Keyboard Report
#[macro_export]
macro_rules! k {
    ($x: tt) => {
        $crate::function::Function::Report($crate::report::Report::Keyboard(
            $crate::report::Keyboard::$x,
        ))
    };
}

// Consumer Report
#[macro_export]
macro_rules! c {
    ($x: tt) => {
        $crate::function::Function::Report($crate::report::Report::Consumer(
            $crate::report::Consumer::$x,
        ))
    };
}

// Desktop Report
#[macro_export]
macro_rules! d {
    ($x: tt) => {
        $crate::function::Function::Report($crate::report::Report::Desktop(
            $crate::report::Desktop::$x,
        ))
    };
}

// Layer Change
#[macro_export]
macro_rules! l {
    ($x: tt) => {{
        $crate::function::Function::Layer($x)
    }};
}
