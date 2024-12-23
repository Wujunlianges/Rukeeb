use crate::event::Event;
use crate::function::Function;

pub mod holdtap;

pub trait Handle: Sync {
    fn handle(&self, event: &Event) -> Option<&Function>;
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
    fn handle(&self, event: &Event) -> Option<&Function> {
        match event {
            Event::Press(_) | Event::Pressed(_) => Some(&self.0),
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
    fn handle(&self, event: &Event) -> Option<&Function> {
        match event {
            Event::Press(_) => Some(&self.0),
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
    fn handle(&self, event: &Event) -> Option<&Function> {
        match event {
            Event::Press(_) => Some(&self.0),
            Event::Release(_) => Some(&self.1),
            _ => None,
        }
    }
}

// Handler Macros

// Keyboard Report Hold
#[macro_export]
macro_rules! kbhd {
    ($x:tt) => {
        $crate::handler::Hold::new($crate::kb!($x))
    };
}

// Consumer Report Tap
#[macro_export]
macro_rules! cutp {
    ($x:tt) => {
        $crate::handler::Tap::new($crate::cu!($x))
    };
}

// Consumer Report Hold
#[macro_export]
macro_rules! cuhd {
    ($x:tt) => {
        $crate::handler::Hold::new($crate::cu!($x))
    };
}

// Desktop Report Tap
#[macro_export]
macro_rules! dktp {
    ($x:tt) => {
        $crate::handler::Tap::new($crate::dk!($x))
    };
}

// Desktop Report Hold
#[macro_export]
macro_rules! dkhd {
    ($x:tt) => {
        $crate::handler::Hold::new($crate::dk!($x))
    };
}

// Layer Tap
#[macro_export]
macro_rules! lytp {
    ($x:tt) => {
        $crate::handler::Tap::new($crate::ly!($x))
    };
}

// Layer OnOff
#[macro_export]
macro_rules! lyoo {
    ($x0:tt, $x1:tt) => {
        $crate::handler::OnOff::new($crate::ly!($x0), $crate::ly!($x1))
    };
}

// Macro for QMK keycodes alias
// kc!($x) = KC_$x
#[macro_export]
#[rustfmt::skip]
macro_rules! kc {
    // Keyboard
    (NO)   => {$crate::kbhd!(NoEventIndicated)};
    (A)    => {$crate::kbhd!(A)};
    (B)    => {$crate::kbhd!(B)};
    (C)    => {$crate::kbhd!(C)};
    (D)    => {$crate::kbhd!(D)};
    (E)    => {$crate::kbhd!(E)};
    (F)    => {$crate::kbhd!(F)};
    (G)    => {$crate::kbhd!(G)};
    (H)    => {$crate::kbhd!(H)};
    (I)    => {$crate::kbhd!(I)};
    (J)    => {$crate::kbhd!(J)};
    (K)    => {$crate::kbhd!(K)};
    (L)    => {$crate::kbhd!(L)};
    (M)    => {$crate::kbhd!(M)};
    (N)    => {$crate::kbhd!(N)};
    (O)    => {$crate::kbhd!(O)};
    (P)    => {$crate::kbhd!(P)};
    (Q)    => {$crate::kbhd!(Q)};
    (R)    => {$crate::kbhd!(R)};
    (S)    => {$crate::kbhd!(S)};
    (T)    => {$crate::kbhd!(T)};
    (U)    => {$crate::kbhd!(U)};
    (V)    => {$crate::kbhd!(V)};
    (W)    => {$crate::kbhd!(W)};
    (X)    => {$crate::kbhd!(X)};
    (Y)    => {$crate::kbhd!(Y)};
    (Z)    => {$crate::kbhd!(Z)};
    (1)    => {$crate::kbhd!(Keyboard1)};
    (2)    => {$crate::kbhd!(Keyboard2)};
    (3)    => {$crate::kbhd!(Keyboard3)};
    (4)    => {$crate::kbhd!(Keyboard4)};
    (5)    => {$crate::kbhd!(Keyboard5)};
    (6)    => {$crate::kbhd!(Keyboard6)};
    (7)    => {$crate::kbhd!(Keyboard7)};
    (8)    => {$crate::kbhd!(Keyboard8)};
    (9)    => {$crate::kbhd!(Keyboard9)};
    (0)    => {$crate::kbhd!(Keyboard0)};
    (ENT)  => {$crate::kbhd!(ReturnEnter)};
    (F1)   => {$crate::kbhd!(F1)};
    (F2)   => {$crate::kbhd!(F2)};
    (F3)   => {$crate::kbhd!(F3)};
    (F4)   => {$crate::kbhd!(F4)};
    (F5)   => {$crate::kbhd!(F5)};
    (F6)   => {$crate::kbhd!(F6)};
    (F7)   => {$crate::kbhd!(F7)};
    (F8)   => {$crate::kbhd!(F8)};
    (F9)   => {$crate::kbhd!(F9)};
    (F10)  => {$crate::kbhd!(F10)};
    (F11)  => {$crate::kbhd!(F11)};
    (F12)  => {$crate::kbhd!(F12)};
    (ENT)  => {$crate::kbhd!(ReturnEnter)};
    (ESC)  => {$crate::kbhd!(Escape)};
    (BSPC) => {$crate::kbhd!(DeleteBackspace)};
    (TAB)  => {$crate::kbhd!(Tab)};
    (SPC)  => {$crate::kbhd!(Space)};
    (MINS) => {$crate::kbhd!(Minus)};
    (EQL)  => {$crate::kbhd!(Equal)};
    (LBRC) => {$crate::kbhd!(LeftBrace)};
    (RBRC) => {$crate::kbhd!(RightBrace)};
    (BSLS) => {$crate::kbhd!(Backslash)};
    (NUHS) => {$crate::kbhd!(NonUSHash)};
    (SCLN) => {$crate::kbhd!(Semicolon)};
    (QUOT) => {$crate::kbhd!(Apostrophe)};
    (GRV)  => {$crate::kbhd!(Grave)};
    (COMM) => {$crate::kbhd!(Comma)};
    (DOT)  => {$crate::kbhd!(Dot)};
    (SLSH) => {$crate::kbhd!(ForwardSlash)};
    (CAPS) => {$crate::kbhd!(CapsLock)};
    (PSCR) => {$crate::kbhd!(PrintScreen)};
    (SCRL) => {$crate::kbhd!(ScrollLock)};
    (PAUS) => {$crate::kbhd!(Pause)};
    (INS)  => {$crate::kbhd!(Insert)};
    (HOME) => {$crate::kbhd!(Home)};
    (PGUP) => {$crate::kbhd!(PageUp)};
    (DEL)  => {$crate::kbhd!(DeleteForward)};
    (END)  => {$crate::kbhd!(End)};
    (PGDN) => {$crate::kbhd!(PageDown)};
    (RGHT) => {$crate::kbhd!(RightArrow)};
    (LEFT) => {$crate::kbhd!(LeftArrow)};
    (DOWN) => {$crate::kbhd!(DownArrow)};
    (UP)   => {$crate::kbhd!(UpArrow)};
    (NUM)  => {$crate::kbhd!(KeypadNumLockAndClear)};
    (PSLS) => {$crate::kbhd!(KeypadDivide)};
    (PAST) => {$crate::kbhd!(KeypadMultiply)};
    (PMNS) => {$crate::kbhd!(KeypadSubtract)};
    (PPLS) => {$crate::kbhd!(KeypadAdd)};
    (PENT) => {$crate::kbhd!(KeypadEnter)};
    (P1)   => {$crate::kbhd!(Keypad1)};
    (P2)   => {$crate::kbhd!(Keypad2)};
    (P3)   => {$crate::kbhd!(Keypad3)};
    (P4)   => {$crate::kbhd!(Keypad4)};
    (P5)   => {$crate::kbhd!(Keypad5)};
    (P6)   => {$crate::kbhd!(Keypad6)};
    (P7)   => {$crate::kbhd!(Keypad7)};
    (P8)   => {$crate::kbhd!(Keypad8)};
    (P9)   => {$crate::kbhd!(Keypad9)};
    (P0)   => {$crate::kbhd!(Keypad0)};
    (PDOT) => {$crate::kbhd!(KeypadDot)};
    (NUBS) => {$crate::kbhd!(NonUSBackslash)};
    (APP)  => {$crate::kbhd!(Application)};
    (PWOR) => {$crate::kbhd!(Power)};
    (PEQL) => {$crate::kbhd!(KeypadEqual)};
    (F13)  => {$crate::kbhd!(F13)};
    (F14)  => {$crate::kbhd!(F14)};
    (F15)  => {$crate::kbhd!(F15)};
    (F16)  => {$crate::kbhd!(F16)};
    (F17)  => {$crate::kbhd!(F17)};
    (F18)  => {$crate::kbhd!(F18)};
    (F19)  => {$crate::kbhd!(F19)};
    (F20)  => {$crate::kbhd!(F20)};
    (F21)  => {$crate::kbhd!(F21)};
    (F22)  => {$crate::kbhd!(F22)};
    (F23)  => {$crate::kbhd!(F23)};
    (F24)  => {$crate::kbhd!(F24)};

    (LCTL) => {$crate::kbhd!(LeftControl)};
    (LSFT) => {$crate::kbhd!(LeftShift)};
    (LALT) => {$crate::kbhd!(LeftAlt)};
    (LGUI) => {$crate::kbhd!(LeftGUI)};
    (RCTL) => {$crate::kbhd!(RightControl)};
    (RSFT) => {$crate::kbhd!(RightShift)};
    (RALT) => {$crate::kbhd!(RightAlt)};
    (RGUI) => {$crate::kbhd!(RightGUI)};


    // Desktop
    (PWR)  => {$crate::dktp!(SystemPowerDown)};
    (SLEP) => {$crate::dktp!(SystemSleep)};
    (WAKE) => {$crate::dktp!(SystemWakeUp)};


    // Consumer
    (MUTE) => {$crate::cutp!(Mute)};
    (VOLU) => {$crate::cuhd!(VolumeIncrement)};
    (VOLD) => {$crate::cuhd!(VolumeDecrement)};
    (MNXT) => {$crate::cuhd!(TrackingIncrement)};
    (MPRV) => {$crate::cuhd!(TrackingDecrement)};
    (MSTP) => {$crate::cutp!(Stop)};
    (MPLY) => {$crate::cutp!(PlayPause)};
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
