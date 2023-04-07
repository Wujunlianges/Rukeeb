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

pub struct PressAction(Action);
pub struct PressedAction(Action);
pub struct ReleaseAction(Action);
pub struct ReleasedAction(Action);
pub struct PressReleaseAction(Action);

impl PressAction {
    pub const fn new(action: Action) -> PressAction {
        PressAction(action)
    }
}

impl PressedAction {
    pub const fn new(action: Action) -> PressedAction {
        PressedAction(action)
    }
}

impl ReleaseAction {
    pub const fn new(action: Action) -> ReleaseAction {
        ReleaseAction(action)
    }
}

impl ReleasedAction {
    pub const fn new(action: Action) -> ReleasedAction {
        ReleasedAction(action)
    }
}

impl PressReleaseAction {
    pub const fn new(action: Action) -> PressReleaseAction {
        PressReleaseAction(action)
    }
}

impl Act for PressAction {
    fn act(&self, event: &Event) -> Option<&Action> {
        match event {
            Event::Press(_) => Some(&self.0),
            _ => None,
        }
    }
}

impl Act for PressedAction {
    fn act(&self, event: &Event) -> Option<&Action> {
        match event {
            Event::Press(_) | Event::Pressed(_) => Some(&self.0),
            _ => None,
        }
    }
}

impl Act for ReleaseAction {
    fn act(&self, event: &Event) -> Option<&Action> {
        match event {
            Event::Release(_) => Some(&self.0),
            _ => None,
        }
    }
}

impl Act for ReleasedAction {
    fn act(&self, event: &Event) -> Option<&Action> {
        match event {
            Event::Release(_) | Event::Released(_) => Some(&self.0),
            _ => None,
        }
    }
}

impl Act for PressReleaseAction {
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

// Layer Default
#[macro_export]
macro_rules! ld {
    ($x: tt) => {{
        $crate::action::Action::Layer($crate::action::Layer::Default($x))
    }};
}

// Layer Current
#[macro_export]
macro_rules! lc {
    ($x: tt) => {{
        $crate::action::Action::Layer($crate::action::Layer::Current($x))
    }};
}

// Action Macros

// Pressed Keyboard Report
#[macro_export]
macro_rules! pdkb {
    ($x:tt) => {
        $crate::action::PressedAction::new($crate::kb!($x))
    };
}

// Press Keyboard Report
#[macro_export]
macro_rules! pcu {
    ($x:tt) => {
        $crate::action::PressAction::new($crate::cu!($x))
    };
}

// Pressed Customer Report
#[macro_export]
macro_rules! pdcu {
    ($x:tt) => {
        $crate::action::PressedAction::new($crate::cu!($x))
    };
}

// Press Desktop Report
#[macro_export]
macro_rules! pdk {
    ($x:tt) => {
        $crate::action::PressAction::new($crate::dk!($x))
    };
}

// Pressed Desktop Report
#[macro_export]
macro_rules! pddk {
    ($x:tt) => {
        $crate::action::PressedAction::new($crate::dk!($x))
    };
}

// Press Layer Default
#[macro_export]
macro_rules! pld {
    ($x:tt) => {
        $crate::action::PressAction::new($crate::ld!($x))
    };
}

// PressRelease Layer Current
#[macro_export]
macro_rules! prlc {
    ($x:tt) => {
        $crate::action::PressReleaseAction::new($crate::lc!($x))
    };
}

// Macro for QMK keycodes alias
// kc!($x) = KC_$x
#[macro_export]
#[rustfmt::skip]
macro_rules! kc {
    // Keyboard
    (NO)   => {$crate::pdkb!(NoEventIndicated)};
    (A)    => {$crate::pdkb!(A)};
    (B)    => {$crate::pdkb!(B)};
    (C)    => {$crate::pdkb!(C)};
    (D)    => {$crate::pdkb!(D)};
    (E)    => {$crate::pdkb!(E)};
    (F)    => {$crate::pdkb!(F)};
    (G)    => {$crate::pdkb!(G)};
    (H)    => {$crate::pdkb!(H)};
    (I)    => {$crate::pdkb!(I)};
    (J)    => {$crate::pdkb!(J)};
    (K)    => {$crate::pdkb!(K)};
    (L)    => {$crate::pdkb!(L)};
    (M)    => {$crate::pdkb!(M)};
    (N)    => {$crate::pdkb!(N)};
    (O)    => {$crate::pdkb!(O)};
    (P)    => {$crate::pdkb!(P)};
    (Q)    => {$crate::pdkb!(Q)};
    (R)    => {$crate::pdkb!(R)};
    (S)    => {$crate::pdkb!(S)};
    (T)    => {$crate::pdkb!(T)};
    (U)    => {$crate::pdkb!(U)};
    (V)    => {$crate::pdkb!(V)};
    (W)    => {$crate::pdkb!(W)};
    (X)    => {$crate::pdkb!(X)};
    (Y)    => {$crate::pdkb!(Y)};
    (Z)    => {$crate::pdkb!(Z)};
    (1)    => {$crate::pdkb!(Keyboard1)};
    (2)    => {$crate::pdkb!(Keyboard2)};
    (3)    => {$crate::pdkb!(Keyboard3)};
    (4)    => {$crate::pdkb!(Keyboard4)};
    (5)    => {$crate::pdkb!(Keyboard5)};
    (6)    => {$crate::pdkb!(Keyboard6)};
    (7)    => {$crate::pdkb!(Keyboard7)};
    (8)    => {$crate::pdkb!(Keyboard8)};
    (9)    => {$crate::pdkb!(Keyboard9)};
    (0)    => {$crate::pdkb!(Keyboard0)};
    (ENT)  => {$crate::pdkb!(ReturnEnter)};
    (F1)   => {$crate::pdkb!(F1)};
    (F2)   => {$crate::pdkb!(F2)};
    (F3)   => {$crate::pdkb!(F3)};
    (F4)   => {$crate::pdkb!(F4)};
    (F5)   => {$crate::pdkb!(F5)};
    (F6)   => {$crate::pdkb!(F6)};
    (F7)   => {$crate::pdkb!(F7)};
    (F8)   => {$crate::pdkb!(F8)};
    (F9)   => {$crate::pdkb!(F9)};
    (F10)  => {$crate::pdkb!(F10)};
    (F11)  => {$crate::pdkb!(F11)};
    (F12)  => {$crate::pdkb!(F12)};
    (ENT)  => {$crate::pdkb!(ReturnEnter)};
    (ESC)  => {$crate::pdkb!(Escape)};
    (BSPC) => {$crate::pdkb!(DeleteBackspace)};
    (TAB)  => {$crate::pdkb!(Tab)};
    (SPC)  => {$crate::pdkb!(Space)};
    (MINS) => {$crate::pdkb!(Minus)};
    (EQL)  => {$crate::pdkb!(Equal)};
    (LBRC) => {$crate::pdkb!(LeftBrace)};
    (RBRC) => {$crate::pdkb!(RightBrace)};
    (BSLS) => {$crate::pdkb!(Backslash)};
    (NUHS) => {$crate::pdkb!(NonUSHash)};
    (SCLN) => {$crate::pdkb!(Semicolon)};
    (QUOT) => {$crate::pdkb!(Apostrophe)};
    (GRV)  => {$crate::pdkb!(Grave)};
    (COMM) => {$crate::pdkb!(Comma)};
    (DOT)  => {$crate::pdkb!(Dot)};
    (SLSH) => {$crate::pdkb!(ForwardSlash)};
    (CAPS) => {$crate::pdkb!(CapsLock)};
    (PSCR) => {$crate::pdkb!(PrintScreen)};
    (SCRL) => {$crate::pdkb!(ScrollLock)};
    (PAUS) => {$crate::pdkb!(Pause)};
    (INS)  => {$crate::pdkb!(Insert)};
    (HOME) => {$crate::pdkb!(Home)};
    (PGUP) => {$crate::pdkb!(PageUp)};
    (DEL)  => {$crate::pdkb!(DeleteForward)};
    (END)  => {$crate::pdkb!(End)};
    (PGDN) => {$crate::pdkb!(PageDown)};
    (RGHT) => {$crate::pdkb!(RightArrow)};
    (LEFT) => {$crate::pdkb!(LeftArrow)};
    (DOWN) => {$crate::pdkb!(DownArrow)};
    (UP)   => {$crate::pdkb!(UpArrow)};
    (NUM)  => {$crate::pdkb!(KeypadNumLockAndClear)};
    (PSLS) => {$crate::pdkb!(KeypadDivide)};
    (PAST) => {$crate::pdkb!(KeypadMultiply)};
    (PMNS) => {$crate::pdkb!(KeypadSubtract)};
    (PPLS) => {$crate::pdkb!(KeypadAdd)};
    (PENT) => {$crate::pdkb!(KeypadEnter)};
    (P1)   => {$crate::pdkb!(Keypad1)};
    (P2)   => {$crate::pdkb!(Keypad2)};
    (P3)   => {$crate::pdkb!(Keypad3)};
    (P4)   => {$crate::pdkb!(Keypad4)};
    (P5)   => {$crate::pdkb!(Keypad5)};
    (P6)   => {$crate::pdkb!(Keypad6)};
    (P7)   => {$crate::pdkb!(Keypad7)};
    (P8)   => {$crate::pdkb!(Keypad8)};
    (P9)   => {$crate::pdkb!(Keypad9)};
    (P0)   => {$crate::pdkb!(Keypad0)};
    (PDOT) => {$crate::pdkb!(KeypadDot)};
    (NUBS) => {$crate::pdkb!(NonUSBackslash)};
    (APP)  => {$crate::pdkb!(Application)};
    (PWOR) => {$crate::pdkb!(Power)};
    (PEQL) => {$crate::pdkb!(KeypadEqual)};
    (F13)  => {$crate::pdkb!(F13)};
    (F14)  => {$crate::pdkb!(F14)};
    (F15)  => {$crate::pdkb!(F15)};
    (F16)  => {$crate::pdkb!(F16)};
    (F17)  => {$crate::pdkb!(F17)};
    (F18)  => {$crate::pdkb!(F18)};
    (F19)  => {$crate::pdkb!(F19)};
    (F20)  => {$crate::pdkb!(F20)};
    (F21)  => {$crate::pdkb!(F21)};
    (F22)  => {$crate::pdkb!(F22)};
    (F23)  => {$crate::pdkb!(F23)};
    (F24)  => {$crate::pdkb!(F24)};

    (LCTL) => {$crate::pdkb!(LeftControl)};
    (LSFT) => {$crate::pdkb!(LeftShift)};
    (LALT) => {$crate::pdkb!(LeftAlt)};
    (LGUI) => {$crate::pdkb!(LeftGUI)};
    (RCTL) => {$crate::pdkb!(RightControl)};
    (RSFT) => {$crate::pdkb!(RightShift)};
    (RALT) => {$crate::pdkb!(RightAlt)};
    (RGUI) => {$crate::pdkb!(RightGUI)};


    // Desktop
    (PWR)  => {$crate::pdk!(SystemPowerDown)};
    (SLEP) => {$crate::pdk!(SystemSleep)};
    (WAKE) => {$crate::pdk!(SystemWakeUp)};


    // Customer
    (MUTE) => {$crate::pcu!(Mute)};
    (VOLU) => {$crate::pdcu!(VolumeIncrement)};
    (VOLD) => {$crate::pdcu!(VolumeDecrement)};
    (MNXT) => {$crate::pdcu!(TrackingIncrement)};
    (MPRV) => {$crate::pdcu!(TrackingDecrement)};
    (MSTP) => {$crate::pcu!(Stop)};
    (MPLY) => {$crate::pcu!(PlayPause)};
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

        pcu!(VolumeIncrement);
        pdcu!(VolumeIncrement);

        ld!(0);
        lc!(0);
        prlc!(0);
        pld!(0);

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
