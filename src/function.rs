use crate::report::Report;

#[derive(Clone, Copy)]
pub enum Function {
    Report(Report),
    Layer(usize),
}

// Function Macros

// Keyboard Report
#[macro_export]
macro_rules! kb {
    ($x: tt) => {
        $crate::function::Function::Report($crate::report::Report::Keyboard(
            $crate::report::Keyboard::$x,
        ))
    };
}

// Consumer Report
#[macro_export]
macro_rules! cu {
    ($x: tt) => {
        $crate::function::Function::Report($crate::report::Report::Consumer(
            $crate::report::Consumer::$x,
        ))
    };
}

// Desktop Report
#[macro_export]
macro_rules! dk {
    ($x: tt) => {
        $crate::function::Function::Report($crate::report::Report::Desktop(
            $crate::report::Desktop::$x,
        ))
    };
}

// Layer Change
#[macro_export]
macro_rules! ly {
    ($x: tt) => {{
        $crate::function::Function::Layer($x)
    }};
}
