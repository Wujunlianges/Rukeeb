pub use usbd_human_interface_device::page::{Consumer, Desktop, Keyboard};

use crate::event::Event;
use crate::report::Report;

pub trait Act: Sync {
    fn act(&self, event: &Event) -> Option<&KeyboardAction>;
}

pub struct Action(pub &'static dyn Act);

impl Action {
    pub fn act(&self, event: &Event) -> Option<&KeyboardAction> {
        self.0.act(event)
    }
}

pub enum KeyboardAction {
    Report(Report),
    LayerAction(LayerAction),
}

pub enum LayerAction {
    DefaultLayer(usize),
    CurrentLayer(usize),
    UndoCurrentLayer(usize),
}

pub struct PressAction(pub KeyboardAction);
pub struct PressedAction(pub KeyboardAction);
pub struct ReleaseAction(pub KeyboardAction);
pub struct ReleasedAction(pub KeyboardAction);

impl Act for PressAction {
    fn act(&self, event: &Event) -> Option<&KeyboardAction> {
        match event {
            Event::Press(_) => Some(&self.0),
            _ => None,
        }
    }
}

impl Act for PressedAction {
    fn act(&self, event: &Event) -> Option<&KeyboardAction> {
        match event {
            Event::Press(_) | Event::Pressed(_) => Some(&self.0),
            _ => None,
        }
    }
}

impl Act for ReleaseAction {
    fn act(&self, event: &Event) -> Option<&KeyboardAction> {
        match event {
            Event::Release(_) => Some(&self.0),
            _ => None,
        }
    }
}

impl Act for ReleasedAction {
    fn act(&self, event: &Event) -> Option<&KeyboardAction> {
        match event {
            Event::Release(_) | Event::Released(_) => Some(&self.0),
            _ => None,
        }
    }
}

// Keyboard Action Macros

// Keyboard Report
#[macro_export]
macro_rules! kb {
    ($x: tt) => {
        $crate::action::KeyboardAction::Report($crate::report::Report::Keyboard(
            $crate::action::Keyboard::$x,
        ))
    };
}

// Customer Report
#[macro_export]
macro_rules! cu {
    ($x: tt) => {
        $crate::action::KeyboardAction::Report($crate::report::Report::Consumer(
            $crate::action::Consumer::$x,
        ))
    };
}

// Desktop Report
#[macro_export]
macro_rules! dk {
    ($x: tt) => {
        $crate::action::KeyboardAction::Report($crate::report::Report::Desktop(
            $crate::action::Desktop::$x,
        ))
    };
}

// LayerAction DefaultLayer
#[macro_export]
macro_rules! ld {
    ($x: tt) => {
        $crate::action::KeyboardAction::LayerAction($crate::action::LayerAction::DefaultLayer($x))
    };
}

// LayerAction CurrentLayer
#[macro_export]
macro_rules! lc {
    ($x: tt) => {
        $crate::action::KeyboardAction::LayerAction($crate::action::LayerAction::CurrentLayer($x))
    };
}

// LayerAction UndoCurrentLayer
#[macro_export]
macro_rules! lu {
    ($x: tt) => {
        $crate::action::KeyboardAction::LayerAction($crate::action::LayerAction::UndoCurrentLayer(
            $x,
        ))
    };
}

// Action Macros

// Pressed Keyboard Report
#[macro_export]
macro_rules! pdkb {
    ($x:tt) => {
        $crate::action::Action(&$crate::action::PressedAction(kb!($x)))
    };
}

// Press Keyboard Report
#[macro_export]
macro_rules! prcu {
    ($x:tt) => {
        $crate::action::Action(&$crate::action::PressAction(cu!($x)))
    };
}

// Pressed Customer Report
#[macro_export]
macro_rules! pdcu {
    ($x:tt) => {
        $crate::action::Action(&$crate::action::PressedAction(cu!($x)))
    };
}

// Press Desktop Report
#[macro_export]
macro_rules! prdk {
    ($x:tt) => {
        $crate::action::Action(&$crate::action::PressAction(dk!($x)))
    };
}

// Pressed Desktop Report
#[macro_export]
macro_rules! pddk {
    ($x:tt) => {
        $crate::action::Action(&$crate::action::PressedAction(dk!($x)))
    };
}

// Press LayerAction DefaultLayer
#[macro_export]
macro_rules! prld {
    ($x:tt) => {
        $crate::action::Action(&$crate::action::PressAction(ld!($x)))
    };
}

// Press LayerAction CurrentLayer
#[macro_export]
macro_rules! prlc {
    ($x:tt) => {
        $crate::action::Action(&$crate::action::PressAction(lc!($x)))
    };
}

// Press LayerAction UndoCurrentLayer
#[macro_export]
macro_rules! relu {
    ($x:tt) => {
        $crate::action::Action(&$crate::action::ReleaseAction(lu!($x)))
    };
}

// Macro for QMK keycodes alias
// kc!($x) = KC_$x
#[macro_export]
#[rustfmt::skip]
macro_rules! kc {
    // Keyboard
    (NO)   => {pdkb!(NoEventIndicated)};
    (A)    => {pdkb!(A)};
    (B)    => {pdkb!(B)};
    (C)    => {pdkb!(C)};
    (D)    => {pdkb!(D)};
    (E)    => {pdkb!(E)};
    (F)    => {pdkb!(F)};
    (G)    => {pdkb!(G)};
    (H)    => {pdkb!(H)};
    (I)    => {pdkb!(I)};
    (J)    => {pdkb!(J)};
    (K)    => {pdkb!(K)};
    (L)    => {pdkb!(L)};
    (M)    => {pdkb!(M)};
    (N)    => {pdkb!(N)};
    (O)    => {pdkb!(O)};
    (P)    => {pdkb!(P)};
    (Q)    => {pdkb!(Q)};
    (R)    => {pdkb!(R)};
    (S)    => {pdkb!(S)};
    (T)    => {pdkb!(T)};
    (U)    => {pdkb!(U)};
    (V)    => {pdkb!(V)};
    (W)    => {pdkb!(W)};
    (X)    => {pdkb!(X)};
    (Y)    => {pdkb!(Y)};
    (Z)    => {pdkb!(Z)};
    (1)    => {pdkb!(Keyboard1)};
    (2)    => {pdkb!(Keyboard2)};
    (3)    => {pdkb!(Keyboard3)};
    (4)    => {pdkb!(Keyboard4)};
    (5)    => {pdkb!(Keyboard5)};
    (6)    => {pdkb!(Keyboard6)};
    (7)    => {pdkb!(Keyboard7)};
    (8)    => {pdkb!(Keyboard8)};
    (9)    => {pdkb!(Keyboard9)};
    (0)    => {pdkb!(Keyboard0)};
    (ENT)  => {pdkb!(ReturnEnter)};
    (F1)   => {pdkb!(F1)};
    (F2)   => {pdkb!(F2)};
    (F3)   => {pdkb!(F3)};
    (F4)   => {pdkb!(F4)};
    (F5)   => {pdkb!(F5)};
    (F6)   => {pdkb!(F6)};
    (F7)   => {pdkb!(F7)};
    (F8)   => {pdkb!(F8)};
    (F9)   => {pdkb!(F9)};
    (F10)  => {pdkb!(F10)};
    (F11)  => {pdkb!(F11)};
    (F12)  => {pdkb!(F12)};
    (ENT)  => {pdkb!(ReturnEnter)};
    (ESC)  => {pdkb!(Escape)};
    (BSPC) => {pdkb!(DeleteBackspace)};
    (TAB)  => {pdkb!(Tab)};
    (SPC)  => {pdkb!(Space)};
    (MINS) => {pdkb!(Minus)};
    (EQL)  => {pdkb!(Equal)};
    (LBRC) => {pdkb!(LeftBrace)};
    (RBRC) => {pdkb!(RightBrace)};
    (BSLS) => {pdkb!(Backslash)};
    (NUHS) => {pdkb!(NonUSHash)};
    (SCLN) => {pdkb!(Semicolon)};
    (QUOT) => {pdkb!(Apostrophe)};
    (GRV)  => {pdkb!(Grave)};
    (COMM) => {pdkb!(Comma)};
    (DOT)  => {pdkb!(Dot)};
    (SLSH) => {pdkb!(ForwardSlash)};
    (CAPS) => {pdkb!(CapsLock)};
    (PSCR) => {pdkb!(PrintScreen)};
    (SCRL) => {pdkb!(ScrollLock)};
    (PAUS) => {pdkb!(Pause)};
    (INS)  => {pdkb!(Insert)};
    (HOME) => {pdkb!(Home)};
    (PGUP) => {pdkb!(PageUp)};
    (DEL)  => {pdkb!(DeleteForward)};
    (END)  => {pdkb!(End)};
    (PGDN) => {pdkb!(PageDown)};
    (RGHT) => {pdkb!(RightArrow)};
    (LEFT) => {pdkb!(LeftArrow)};
    (DOWN) => {pdkb!(DownArrow)};
    (UP)   => {pdkb!(UpArrow)};
    (NUM)  => {pdkb!(KeypadNumLockAndClear)};
    (PSLS) => {pdkb!(KeypadDivide)};
    (PAST) => {pdkb!(KeypadMultiply)};
    (PMNS) => {pdkb!(KeypadSubtract)};
    (PPLS) => {pdkb!(KeypadAdd)};
    (PENT) => {pdkb!(KeypadEnter)};
    (P1)   => {pdkb!(Keypad1)};
    (P2)   => {pdkb!(Keypad2)};
    (P3)   => {pdkb!(Keypad3)};
    (P4)   => {pdkb!(Keypad4)};
    (P5)   => {pdkb!(Keypad5)};
    (P6)   => {pdkb!(Keypad6)};
    (P7)   => {pdkb!(Keypad7)};
    (P8)   => {pdkb!(Keypad8)};
    (P9)   => {pdkb!(Keypad9)};
    (P0)   => {pdkb!(Keypad0)};
    (PDOT) => {pdkb!(KeypadDot)};
    (NUBS) => {pdkb!(NonUSBackslash)};
    (APP)  => {pdkb!(Application)};
    (PWOR) => {pdkb!(Power)};
    (PEQL) => {pdkb!(KeypadEqual)};
    (F13)  => {pdkb!(F13)};
    (F14)  => {pdkb!(F14)};
    (F15)  => {pdkb!(F15)};
    (F16)  => {pdkb!(F16)};
    (F17)  => {pdkb!(F17)};
    (F18)  => {pdkb!(F18)};
    (F19)  => {pdkb!(F19)};
    (F20)  => {pdkb!(F20)};
    (F21)  => {pdkb!(F21)};
    (F22)  => {pdkb!(F22)};
    (F23)  => {pdkb!(F23)};
    (F24)  => {pdkb!(F24)};

    (LCTL) => {pdkb!(LeftControl)};
    (LSFT) => {pdkb!(LeftShift)};
    (LALT) => {pdkb!(LeftAlt)};
    (LGUI) => {pdkb!(LeftGUI)};
    (RCTL) => {pdkb!(RightControl)};
    (RSFT) => {pdkb!(RightShift)};
    (RALT) => {pdkb!(RightAlt)};
    (RGUI) => {pdkb!(RightGUI)};


    // Desktop
    (PWR)  => {prdk!(SystemPowerDown)};
    (SLEP) => {prdk!(SystemSleep)};
    (WAKE) => {prdk!(SystemWakeUp)};


    // Customer
    (MUTE) => {prcu!(Mute)};
    (VOLU) => {pdcu!(VolumeIncrement)};
    (VOLD) => {pdcu!(VolumeDecrement)};
    (MNXT) => {pdcu!(TrackingIncrement)};
    (MPRV) => {pdcu!(TrackingDecrement)};
    (MSTP) => {prcu!(Stop)};
    (MPLY) => {prcu!(PlayPause)};
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_all_macros() {
        kc!(A);

        kb!(A);
        cu!(VolumeIncrement);

        pdkb!(A);

        prcu!(VolumeIncrement);
        pdcu!(VolumeIncrement);

        lu!(0);
        ld!(0);
        lc!(0);
        prlc!(0);
        prld!(0);
        relu!(0);

        kc!(NO);
        kc!(A);
        kc!(B);
        kc!(C);
        kc!(D);
        kc!(E);
        kc!(F);
        kc!(G);
        kc!(H);
        kc!(I);
        kc!(J);
        kc!(K);
        kc!(L);
        kc!(M);
        kc!(N);
        kc!(O);
        kc!(P);
        kc!(Q);
        kc!(R);
        kc!(S);
        kc!(T);
        kc!(U);
        kc!(V);
        kc!(W);
        kc!(X);
        kc!(Y);
        kc!(Z);
        kc!(1);
        kc!(2);
        kc!(3);
        kc!(4);
        kc!(5);
        kc!(6);
        kc!(7);
        kc!(8);
        kc!(9);
        kc!(0);
        kc!(ENT);
        kc!(F1);
        kc!(F2);
        kc!(F3);
        kc!(F4);
        kc!(F5);
        kc!(F6);
        kc!(F7);
        kc!(F8);
        kc!(F9);
        kc!(F10);
        kc!(F11);
        kc!(F12);
        kc!(ENT);
        kc!(ESC);
        kc!(BSPC);
        kc!(TAB);
        kc!(SPC);
        kc!(MINS);
        kc!(EQL);
        kc!(LBRC);
        kc!(RBRC);
        kc!(BSLS);
        kc!(NUHS);
        kc!(SCLN);
        kc!(QUOT);
        kc!(GRV);
        kc!(COMM);
        kc!(DOT);
        kc!(SLSH);
        kc!(CAPS);
        kc!(PSCR);
        kc!(SCRL);
        kc!(PAUS);
        kc!(INS);
        kc!(HOME);
        kc!(PGUP);
        kc!(DEL);
        kc!(END);
        kc!(PGDN);
        kc!(RGHT);
        kc!(LEFT);
        kc!(DOWN);
        kc!(UP);
        kc!(NUM);
        kc!(PSLS);
        kc!(PAST);
        kc!(PMNS);
        kc!(PPLS);
        kc!(PENT);
        kc!(P1);
        kc!(P2);
        kc!(P3);
        kc!(P4);
        kc!(P5);
        kc!(P6);
        kc!(P7);
        kc!(P8);
        kc!(P9);
        kc!(P0);
        kc!(PDOT);
        kc!(NUBS);
        kc!(APP);
        kc!(PWOR);
        kc!(PEQL);
        kc!(F13);
        kc!(F14);
        kc!(F15);
        kc!(F16);
        kc!(F17);
        kc!(F18);
        kc!(F19);
        kc!(F20);
        kc!(F21);
        kc!(F22);
        kc!(F23);
        kc!(F24);
        kc!(LCTL);
        kc!(LSFT);
        kc!(LALT);
        kc!(LGUI);
        kc!(RCTL);
        kc!(RSFT);
        kc!(RALT);
        kc!(RGUI);
        kc!(PWR);
        kc!(SLEP);
        kc!(WAKE);
        kc!(MUTE);
        kc!(VOLU);
        kc!(VOLD);
        kc!(MNXT);
        kc!(MPRV);
        kc!(MSTP);
        kc!(MPLY);
    }
}
