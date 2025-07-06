use crate::event::Event;
use crate::function::Function;

pub mod holdtap;

pub trait Handle: Sync {
    fn handle(&self, event: &Event) -> Option<Function>;
}

pub struct Hold(Function);
pub struct Tap(Function);
pub struct OnOff(Function, Function);

impl Hold {
    pub const fn new(f: Function) -> Hold {
        Hold(f)
    }
}

impl Handle for Hold {
    fn handle(&self, event: &Event) -> Option<Function> {
        match event {
            Event::Pressing(_) | Event::Pressed(_) => Some(self.0),
            _ => None,
        }
    }
}

impl Tap {
    pub const fn new(f: Function) -> Tap {
        Tap(f)
    }
}

impl Handle for Tap {
    fn handle(&self, event: &Event) -> Option<Function> {
        match event {
            Event::Pressing(_) => Some(self.0),
            _ => None,
        }
    }
}

impl OnOff {
    pub const fn new(f0: Function, f1: Function) -> OnOff {
        OnOff(f0, f1)
    }
}

impl Handle for OnOff {
    fn handle(&self, event: &Event) -> Option<Function> {
        match event {
            Event::Pressing(_) => Some(self.0),
            Event::Releasing(_) => Some(self.1),
            _ => None,
        }
    }
}

// Handler Macros

// Keyboard Report Hold
#[macro_export]
macro_rules! kh {
    ($x:tt) => {
        $crate::handler::Hold::new($crate::k!($x))
    };
}

// Consumer Report Tap
#[macro_export]
macro_rules! ct {
    ($x:tt) => {
        $crate::handler::Tap::new($crate::c!($x))
    };
}

// Consumer Report Hold
#[macro_export]
macro_rules! ch {
    ($x:tt) => {
        $crate::handler::Hold::new($crate::c!($x))
    };
}

// Desktop Report Tap
#[macro_export]
macro_rules! dt {
    ($x:tt) => {
        $crate::handler::Tap::new($crate::d!($x))
    };
}

// Desktop Report Hold
#[macro_export]
macro_rules! dh {
    ($x:tt) => {
        $crate::handler::Hold::new($crate::d!($x))
    };
}

// Layer Tap
#[macro_export]
macro_rules! lt {
    ($x:tt) => {
        $crate::handler::Tap::new($crate::l!($x))
    };
}

// Layer OnOff
#[macro_export]
macro_rules! lo {
    ($x0:tt, $x1:tt) => {
        $crate::handler::OnOff::new($crate::l!($x0), $crate::l!($x1))
    };
}

// Macro for QMK keycodes alias
// kc!($x) = KC_$x
#[macro_export]
#[rustfmt::skip]
macro_rules! kc {
    // Keyboard
    (NO)   => {$crate::kh!(NoEventIndicated)};
    (A)    => {$crate::kh!(A)};
    (B)    => {$crate::kh!(B)};
    (C)    => {$crate::kh!(C)};
    (D)    => {$crate::kh!(D)};
    (E)    => {$crate::kh!(E)};
    (F)    => {$crate::kh!(F)};
    (G)    => {$crate::kh!(G)};
    (H)    => {$crate::kh!(H)};
    (I)    => {$crate::kh!(I)};
    (J)    => {$crate::kh!(J)};
    (K)    => {$crate::kh!(K)};
    (L)    => {$crate::kh!(L)};
    (M)    => {$crate::kh!(M)};
    (N)    => {$crate::kh!(N)};
    (O)    => {$crate::kh!(O)};
    (P)    => {$crate::kh!(P)};
    (Q)    => {$crate::kh!(Q)};
    (R)    => {$crate::kh!(R)};
    (S)    => {$crate::kh!(S)};
    (T)    => {$crate::kh!(T)};
    (U)    => {$crate::kh!(U)};
    (V)    => {$crate::kh!(V)};
    (W)    => {$crate::kh!(W)};
    (X)    => {$crate::kh!(X)};
    (Y)    => {$crate::kh!(Y)};
    (Z)    => {$crate::kh!(Z)};
    (1)    => {$crate::kh!(Keyboard1)};
    (2)    => {$crate::kh!(Keyboard2)};
    (3)    => {$crate::kh!(Keyboard3)};
    (4)    => {$crate::kh!(Keyboard4)};
    (5)    => {$crate::kh!(Keyboard5)};
    (6)    => {$crate::kh!(Keyboard6)};
    (7)    => {$crate::kh!(Keyboard7)};
    (8)    => {$crate::kh!(Keyboard8)};
    (9)    => {$crate::kh!(Keyboard9)};
    (0)    => {$crate::kh!(Keyboard0)};
    (ENT)  => {$crate::kh!(ReturnEnter)};
    (F1)   => {$crate::kh!(F1)};
    (F2)   => {$crate::kh!(F2)};
    (F3)   => {$crate::kh!(F3)};
    (F4)   => {$crate::kh!(F4)};
    (F5)   => {$crate::kh!(F5)};
    (F6)   => {$crate::kh!(F6)};
    (F7)   => {$crate::kh!(F7)};
    (F8)   => {$crate::kh!(F8)};
    (F9)   => {$crate::kh!(F9)};
    (F10)  => {$crate::kh!(F10)};
    (F11)  => {$crate::kh!(F11)};
    (F12)  => {$crate::kh!(F12)};
    (ENT)  => {$crate::kh!(ReturnEnter)};
    (ESC)  => {$crate::kh!(Escape)};
    (BSPC) => {$crate::kh!(DeleteBackspace)};
    (TAB)  => {$crate::kh!(Tab)};
    (SPC)  => {$crate::kh!(Space)};
    (MINS) => {$crate::kh!(Minus)};
    (EQL)  => {$crate::kh!(Equal)};
    (LBRC) => {$crate::kh!(LeftBrace)};
    (RBRC) => {$crate::kh!(RightBrace)};
    (BSLS) => {$crate::kh!(Backslash)};
    (NUHS) => {$crate::kh!(NonUSHash)};
    (SCLN) => {$crate::kh!(Semicolon)};
    (QUOT) => {$crate::kh!(Apostrophe)};
    (GRV)  => {$crate::kh!(Grave)};
    (COMM) => {$crate::kh!(Comma)};
    (DOT)  => {$crate::kh!(Dot)};
    (SLSH) => {$crate::kh!(ForwardSlash)};
    (CAPS) => {$crate::kh!(CapsLock)};
    (PSCR) => {$crate::kh!(PrintScreen)};
    (SCRL) => {$crate::kh!(ScrollLock)};
    (PAUS) => {$crate::kh!(Pause)};
    (INS)  => {$crate::kh!(Insert)};
    (HOME) => {$crate::kh!(Home)};
    (PGUP) => {$crate::kh!(PageUp)};
    (DEL)  => {$crate::kh!(DeleteForward)};
    (END)  => {$crate::kh!(End)};
    (PGDN) => {$crate::kh!(PageDown)};
    (RGHT) => {$crate::kh!(RightArrow)};
    (LEFT) => {$crate::kh!(LeftArrow)};
    (DOWN) => {$crate::kh!(DownArrow)};
    (UP)   => {$crate::kh!(UpArrow)};
    (NUM)  => {$crate::kh!(KeypadNumLockAndClear)};
    (PSLS) => {$crate::kh!(KeypadDivide)};
    (PAST) => {$crate::kh!(KeypadMultiply)};
    (PMNS) => {$crate::kh!(KeypadSubtract)};
    (PPLS) => {$crate::kh!(KeypadAdd)};
    (PENT) => {$crate::kh!(KeypadEnter)};
    (P1)   => {$crate::kh!(Keypad1)};
    (P2)   => {$crate::kh!(Keypad2)};
    (P3)   => {$crate::kh!(Keypad3)};
    (P4)   => {$crate::kh!(Keypad4)};
    (P5)   => {$crate::kh!(Keypad5)};
    (P6)   => {$crate::kh!(Keypad6)};
    (P7)   => {$crate::kh!(Keypad7)};
    (P8)   => {$crate::kh!(Keypad8)};
    (P9)   => {$crate::kh!(Keypad9)};
    (P0)   => {$crate::kh!(Keypad0)};
    (PDOT) => {$crate::kh!(KeypadDot)};
    (NUBS) => {$crate::kh!(NonUSBackslash)};
    (APP)  => {$crate::kh!(Application)};
    (PWOR) => {$crate::kh!(Power)};
    (PEQL) => {$crate::kh!(KeypadEqual)};
    (F13)  => {$crate::kh!(F13)};
    (F14)  => {$crate::kh!(F14)};
    (F15)  => {$crate::kh!(F15)};
    (F16)  => {$crate::kh!(F16)};
    (F17)  => {$crate::kh!(F17)};
    (F18)  => {$crate::kh!(F18)};
    (F19)  => {$crate::kh!(F19)};
    (F20)  => {$crate::kh!(F20)};
    (F21)  => {$crate::kh!(F21)};
    (F22)  => {$crate::kh!(F22)};
    (F23)  => {$crate::kh!(F23)};
    (F24)  => {$crate::kh!(F24)};

    (LCTL) => {$crate::kh!(LeftControl)};
    (LSFT) => {$crate::kh!(LeftShift)};
    (LALT) => {$crate::kh!(LeftAlt)};
    (LGUI) => {$crate::kh!(LeftGUI)};
    (RCTL) => {$crate::kh!(RightControl)};
    (RSFT) => {$crate::kh!(RightShift)};
    (RALT) => {$crate::kh!(RightAlt)};
    (RGUI) => {$crate::kh!(RightGUI)};


    // Desktop
    (PWR)  => {$crate::dt!(SystemPowerDown)};
    (SLEP) => {$crate::dt!(SystemSleep)};
    (WAKE) => {$crate::dt!(SystemWakeUp)};


    // Consumer
    (MUTE) => {$crate::ct!(Mute)};
    (VOLU) => {$crate::ch!(VolumeIncrement)};
    (VOLD) => {$crate::ch!(VolumeDecrement)};
    (MNXT) => {$crate::ch!(TrackingIncrement)};
    (MPRV) => {$crate::ch!(TrackingDecrement)};
    (MSTP) => {$crate::ct!(Stop)};
    (MPLY) => {$crate::ct!(PlayPause)};
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
        let _handlers: [&dyn crate::handler::Handle; 132] = test_kc![
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
