use crate::event::Event;
use crate::report::Report;

pub mod holdtap;

pub trait Act: Sync {
    fn act(&self, event: &Event) -> Option<&Action>;
}

pub enum Action {
    Report(Report),
    Layer(Layer),
}

pub enum Layer {
    Default(usize),
    Current(usize),
}

pub struct Hold(Action);
pub struct Tap(Action);
pub struct OnOff(Action);

macro_rules! implement_new {
    ($($x:ident),* $(,)?) => {
        $(
            impl $x {
                pub const fn new(action: Action) -> $x {
                    $x(action)
                }
            }
        )*
    };
}

implement_new!(Hold, Tap, OnOff);

impl Act for Hold {
    fn act(&self, event: &Event) -> Option<&Action> {
        match event {
            Event::Press(_) | Event::Pressed(_) => Some(&self.0),
            _ => None,
        }
    }
}

impl Act for Tap {
    fn act(&self, event: &Event) -> Option<&Action> {
        match event {
            Event::Press(_) => Some(&self.0),
            _ => None,
        }
    }
}

impl Act for OnOff {
    fn act(&self, event: &Event) -> Option<&Action> {
        match event {
            Event::Press(_) | Event::Release(_) => Some(&self.0),
            _ => None,
        }
    }
}

// Action Macros

// Keyboard Report
#[macro_export]
macro_rules! kb {
    ($x: tt) => {
        $crate::action::Action::Report($crate::report::Report::Keyboard(
            $crate::report::Keyboard::$x,
        ))
    };
}

// Customer Report
#[macro_export]
macro_rules! cu {
    ($x: tt) => {
        $crate::action::Action::Report($crate::report::Report::Consumer(
            $crate::report::Consumer::$x,
        ))
    };
}

// Desktop Report
#[macro_export]
macro_rules! dk {
    ($x: tt) => {
        $crate::action::Action::Report($crate::report::Report::Desktop($crate::report::Desktop::$x))
    };
}

// Default Layer
#[macro_export]
macro_rules! dl {
    ($x: tt) => {{
        $crate::action::Action::Layer($crate::action::Layer::Default($x))
    }};
}

// Current Layer
#[macro_export]
macro_rules! cl {
    ($x: tt) => {{
        $crate::action::Action::Layer($crate::action::Layer::Current($x))
    }};
}

// Action Macros

// Hold Keyboard Report
#[macro_export]
macro_rules! hdkb {
    ($x:tt) => {
        $crate::action::Hold::new($crate::kb!($x))
    };
}

// Tap Customer Report
#[macro_export]
macro_rules! tpcu {
    ($x:tt) => {
        $crate::action::Tap::new($crate::cu!($x))
    };
}

// Hold Customer Report
#[macro_export]
macro_rules! hdcu {
    ($x:tt) => {
        $crate::action::Hold::new($crate::cu!($x))
    };
}

// Tap Desktop Report
#[macro_export]
macro_rules! tpdk {
    ($x:tt) => {
        $crate::action::Tap::new($crate::dk!($x))
    };
}

// Hold Desktop Report
#[macro_export]
macro_rules! hddk {
    ($x:tt) => {
        $crate::action::Hold::new($crate::dk!($x))
    };
}

// Tap Default Layer
#[macro_export]
macro_rules! tpdl {
    ($x:tt) => {
        $crate::action::Tap::new($crate::dl!($x))
    };
}

// OnOff Current Layer
#[macro_export]
macro_rules! oocl {
    ($x:tt) => {
        $crate::action::OnOff::new($crate::cl!($x))
    };
}

// Macro for QMK keycodes alias
// kc!($x) = KC_$x
#[macro_export]
#[rustfmt::skip]
macro_rules! kc {
    // Keyboard
    (NO)   => {$crate::hdkb!(NoEventIndicated)};
    (A)    => {$crate::hdkb!(A)};
    (B)    => {$crate::hdkb!(B)};
    (C)    => {$crate::hdkb!(C)};
    (D)    => {$crate::hdkb!(D)};
    (E)    => {$crate::hdkb!(E)};
    (F)    => {$crate::hdkb!(F)};
    (G)    => {$crate::hdkb!(G)};
    (H)    => {$crate::hdkb!(H)};
    (I)    => {$crate::hdkb!(I)};
    (J)    => {$crate::hdkb!(J)};
    (K)    => {$crate::hdkb!(K)};
    (L)    => {$crate::hdkb!(L)};
    (M)    => {$crate::hdkb!(M)};
    (N)    => {$crate::hdkb!(N)};
    (O)    => {$crate::hdkb!(O)};
    (P)    => {$crate::hdkb!(P)};
    (Q)    => {$crate::hdkb!(Q)};
    (R)    => {$crate::hdkb!(R)};
    (S)    => {$crate::hdkb!(S)};
    (T)    => {$crate::hdkb!(T)};
    (U)    => {$crate::hdkb!(U)};
    (V)    => {$crate::hdkb!(V)};
    (W)    => {$crate::hdkb!(W)};
    (X)    => {$crate::hdkb!(X)};
    (Y)    => {$crate::hdkb!(Y)};
    (Z)    => {$crate::hdkb!(Z)};
    (1)    => {$crate::hdkb!(Keyboard1)};
    (2)    => {$crate::hdkb!(Keyboard2)};
    (3)    => {$crate::hdkb!(Keyboard3)};
    (4)    => {$crate::hdkb!(Keyboard4)};
    (5)    => {$crate::hdkb!(Keyboard5)};
    (6)    => {$crate::hdkb!(Keyboard6)};
    (7)    => {$crate::hdkb!(Keyboard7)};
    (8)    => {$crate::hdkb!(Keyboard8)};
    (9)    => {$crate::hdkb!(Keyboard9)};
    (0)    => {$crate::hdkb!(Keyboard0)};
    (ENT)  => {$crate::hdkb!(ReturnEnter)};
    (F1)   => {$crate::hdkb!(F1)};
    (F2)   => {$crate::hdkb!(F2)};
    (F3)   => {$crate::hdkb!(F3)};
    (F4)   => {$crate::hdkb!(F4)};
    (F5)   => {$crate::hdkb!(F5)};
    (F6)   => {$crate::hdkb!(F6)};
    (F7)   => {$crate::hdkb!(F7)};
    (F8)   => {$crate::hdkb!(F8)};
    (F9)   => {$crate::hdkb!(F9)};
    (F10)  => {$crate::hdkb!(F10)};
    (F11)  => {$crate::hdkb!(F11)};
    (F12)  => {$crate::hdkb!(F12)};
    (ENT)  => {$crate::hdkb!(ReturnEnter)};
    (ESC)  => {$crate::hdkb!(Escape)};
    (BSPC) => {$crate::hdkb!(DeleteBackspace)};
    (TAB)  => {$crate::hdkb!(Tab)};
    (SPC)  => {$crate::hdkb!(Space)};
    (MINS) => {$crate::hdkb!(Minus)};
    (EQL)  => {$crate::hdkb!(Equal)};
    (LBRC) => {$crate::hdkb!(LeftBrace)};
    (RBRC) => {$crate::hdkb!(RightBrace)};
    (BSLS) => {$crate::hdkb!(Backslash)};
    (NUHS) => {$crate::hdkb!(NonUSHash)};
    (SCLN) => {$crate::hdkb!(Semicolon)};
    (QUOT) => {$crate::hdkb!(Apostrophe)};
    (GRV)  => {$crate::hdkb!(Grave)};
    (COMM) => {$crate::hdkb!(Comma)};
    (DOT)  => {$crate::hdkb!(Dot)};
    (SLSH) => {$crate::hdkb!(ForwardSlash)};
    (CAPS) => {$crate::hdkb!(CapsLock)};
    (PSCR) => {$crate::hdkb!(PrintScreen)};
    (SCRL) => {$crate::hdkb!(ScrollLock)};
    (PAUS) => {$crate::hdkb!(Pause)};
    (INS)  => {$crate::hdkb!(Insert)};
    (HOME) => {$crate::hdkb!(Home)};
    (PGUP) => {$crate::hdkb!(PageUp)};
    (DEL)  => {$crate::hdkb!(DeleteForward)};
    (END)  => {$crate::hdkb!(End)};
    (PGDN) => {$crate::hdkb!(PageDown)};
    (RGHT) => {$crate::hdkb!(RightArrow)};
    (LEFT) => {$crate::hdkb!(LeftArrow)};
    (DOWN) => {$crate::hdkb!(DownArrow)};
    (UP)   => {$crate::hdkb!(UpArrow)};
    (NUM)  => {$crate::hdkb!(KeypadNumLockAndClear)};
    (PSLS) => {$crate::hdkb!(KeypadDivide)};
    (PAST) => {$crate::hdkb!(KeypadMultiply)};
    (PMNS) => {$crate::hdkb!(KeypadSubtract)};
    (PPLS) => {$crate::hdkb!(KeypadAdd)};
    (PENT) => {$crate::hdkb!(KeypadEnter)};
    (P1)   => {$crate::hdkb!(Keypad1)};
    (P2)   => {$crate::hdkb!(Keypad2)};
    (P3)   => {$crate::hdkb!(Keypad3)};
    (P4)   => {$crate::hdkb!(Keypad4)};
    (P5)   => {$crate::hdkb!(Keypad5)};
    (P6)   => {$crate::hdkb!(Keypad6)};
    (P7)   => {$crate::hdkb!(Keypad7)};
    (P8)   => {$crate::hdkb!(Keypad8)};
    (P9)   => {$crate::hdkb!(Keypad9)};
    (P0)   => {$crate::hdkb!(Keypad0)};
    (PDOT) => {$crate::hdkb!(KeypadDot)};
    (NUBS) => {$crate::hdkb!(NonUSBackslash)};
    (APP)  => {$crate::hdkb!(Application)};
    (PWOR) => {$crate::hdkb!(Power)};
    (PEQL) => {$crate::hdkb!(KeypadEqual)};
    (F13)  => {$crate::hdkb!(F13)};
    (F14)  => {$crate::hdkb!(F14)};
    (F15)  => {$crate::hdkb!(F15)};
    (F16)  => {$crate::hdkb!(F16)};
    (F17)  => {$crate::hdkb!(F17)};
    (F18)  => {$crate::hdkb!(F18)};
    (F19)  => {$crate::hdkb!(F19)};
    (F20)  => {$crate::hdkb!(F20)};
    (F21)  => {$crate::hdkb!(F21)};
    (F22)  => {$crate::hdkb!(F22)};
    (F23)  => {$crate::hdkb!(F23)};
    (F24)  => {$crate::hdkb!(F24)};

    (LCTL) => {$crate::hdkb!(LeftControl)};
    (LSFT) => {$crate::hdkb!(LeftShift)};
    (LALT) => {$crate::hdkb!(LeftAlt)};
    (LGUI) => {$crate::hdkb!(LeftGUI)};
    (RCTL) => {$crate::hdkb!(RightControl)};
    (RSFT) => {$crate::hdkb!(RightShift)};
    (RALT) => {$crate::hdkb!(RightAlt)};
    (RGUI) => {$crate::hdkb!(RightGUI)};


    // Desktop
    (PWR)  => {$crate::tpdk!(SystemPowerDown)};
    (SLEP) => {$crate::tpdk!(SystemSleep)};
    (WAKE) => {$crate::tpdk!(SystemWakeUp)};


    // Customer
    (MUTE) => {$crate::tpcu!(Mute)};
    (VOLU) => {$crate::hdcu!(VolumeIncrement)};
    (VOLD) => {$crate::hdcu!(VolumeDecrement)};
    (MNXT) => {$crate::hdcu!(TrackingIncrement)};
    (MPRV) => {$crate::hdcu!(TrackingDecrement)};
    (MSTP) => {$crate::tpcu!(Stop)};
    (MPLY) => {$crate::tpcu!(PlayPause)};
}

#[cfg(test)]
mod test {
    macro_rules! test_kc {
        ($($x:tt),* $(,)?) => {
            [$(&kc!($x),)*]
        };
    }

    #[test]
    fn test_kc() {
        let _actions: [&dyn crate::action::Act; 132] = test_kc![
            NO, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, 1, 2,
            3, 4, 5, 6, 7, 8, 9, 0, ENT, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, ENT,
            ESC, BSPC, TAB, SPC, MINS, EQL, LBRC, RBRC, BSLS, NUHS, SCLN, QUOT, GRV, COMM, DOT,
            SLSH, CAPS, PSCR, SCRL, PAUS, INS, HOME, PGUP, DEL, END, PGDN, RGHT, LEFT, DOWN, UP,
            NUM, PSLS, PAST, PMNS, PPLS, PENT, P1, P2, P3, P4, P5, P6, P7, P8, P9, P0, PDOT, NUBS,
            APP, PWOR, PEQL, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, LCTL,
            LSFT, LALT, LGUI, RCTL, RSFT, RALT, RGUI, PWR, SLEP, WAKE, MUTE, VOLU, VOLD, MNXT,
            MPRV, MSTP, MPLY,
        ];
    }
}
