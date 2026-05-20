pub use usbd_human_interface_device::page::{Consumer, Desktop, Keyboard};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Report {
    Keyboard(Keyboard),
    Consumer(Consumer),
    Desktop(Desktop),
    Custom(u8),
}

// Report Macros
// Macro for QMK keycodes alias
// $rpt!($x) = KC_$x
#[macro_export]
#[rustfmt::skip]
macro_rules! rpt {
    // Private helper for Keyboard Report
    (@kb_rpt $x:tt) => {
        $crate::report::Report::Keyboard($crate::report::Keyboard::$x)
    };

    // Private helper for Consumer Report
    (@co_rpt $x:tt) => {
        $crate::report::Report::Consumer($crate::report::Consumer::$x)
    };

    // Private helper for Desktop Report
    (@dt_rpt $x:tt) => {
        $crate::report::Report::Desktop($crate::report::Desktop::$x)
    };

    // Private helper for Custom Report
    (@cu_rpt $x:tt) => {
        $crate::report::Report::Custom($x)
    };

    // Keyboard
    (A) => {$crate::rpt!(@kb_rpt A)}; (B) => {$crate::rpt!(@kb_rpt B)}; (C) => {$crate::rpt!(@kb_rpt C)}; (D) => {$crate::rpt!(@kb_rpt D)};
    (E) => {$crate::rpt!(@kb_rpt E)}; (F) => {$crate::rpt!(@kb_rpt F)}; (G) => {$crate::rpt!(@kb_rpt G)}; (H) => {$crate::rpt!(@kb_rpt H)};
    (I) => {$crate::rpt!(@kb_rpt I)}; (J) => {$crate::rpt!(@kb_rpt J)}; (K) => {$crate::rpt!(@kb_rpt K)}; (L) => {$crate::rpt!(@kb_rpt L)};
    (M) => {$crate::rpt!(@kb_rpt M)}; (N) => {$crate::rpt!(@kb_rpt N)}; (O) => {$crate::rpt!(@kb_rpt O)}; (P) => {$crate::rpt!(@kb_rpt P)};
    (Q) => {$crate::rpt!(@kb_rpt Q)}; (R) => {$crate::rpt!(@kb_rpt R)}; (S) => {$crate::rpt!(@kb_rpt S)}; (T) => {$crate::rpt!(@kb_rpt T)};
    (U) => {$crate::rpt!(@kb_rpt U)}; (V) => {$crate::rpt!(@kb_rpt V)}; (W) => {$crate::rpt!(@kb_rpt W)}; (X) => {$crate::rpt!(@kb_rpt X)};
    (Y) => {$crate::rpt!(@kb_rpt Y)}; (Z) => {$crate::rpt!(@kb_rpt Z)};

    (0) => {$crate::rpt!(@kb_rpt Keyboard0)}; (1) => {$crate::rpt!(@kb_rpt Keyboard1)}; (2) => {$crate::rpt!(@kb_rpt Keyboard2)};
    (3) => {$crate::rpt!(@kb_rpt Keyboard3)}; (4) => {$crate::rpt!(@kb_rpt Keyboard4)}; (5) => {$crate::rpt!(@kb_rpt Keyboard5)};
    (6) => {$crate::rpt!(@kb_rpt Keyboard6)}; (7) => {$crate::rpt!(@kb_rpt Keyboard7)}; (8) => {$crate::rpt!(@kb_rpt Keyboard8)};
    (9) => {$crate::rpt!(@kb_rpt Keyboard9)};

    (F1)  => {$crate::rpt!(@kb_rpt F1)};  (F2)  => {$crate::rpt!(@kb_rpt F2)};  (F3)  => {$crate::rpt!(@kb_rpt F3)};  (F4)  => {$crate::rpt!(@kb_rpt F4)};
    (F5)  => {$crate::rpt!(@kb_rpt F5)};  (F6)  => {$crate::rpt!(@kb_rpt F6)};  (F7)  => {$crate::rpt!(@kb_rpt F7)};  (F8)  => {$crate::rpt!(@kb_rpt F8)};
    (F9)  => {$crate::rpt!(@kb_rpt F9)};  (F10) => {$crate::rpt!(@kb_rpt F10)}; (F11) => {$crate::rpt!(@kb_rpt F11)}; (F12) => {$crate::rpt!(@kb_rpt F12)};
    (F13) => {$crate::rpt!(@kb_rpt F13)}; (F14) => {$crate::rpt!(@kb_rpt F14)}; (F15) => {$crate::rpt!(@kb_rpt F15)}; (F16) => {$crate::rpt!(@kb_rpt F16)};
    (F17) => {$crate::rpt!(@kb_rpt F17)}; (F18) => {$crate::rpt!(@kb_rpt F18)}; (F19) => {$crate::rpt!(@kb_rpt F19)}; (F20) => {$crate::rpt!(@kb_rpt F20)};
    (F21) => {$crate::rpt!(@kb_rpt F21)}; (F22) => {$crate::rpt!(@kb_rpt F22)}; (F23) => {$crate::rpt!(@kb_rpt F23)}; (F24) => {$crate::rpt!(@kb_rpt F24)};

    (LALT) => {$crate::rpt!(@kb_rpt LeftAlt)};  (LCTL) => {$crate::rpt!(@kb_rpt LeftControl)};  (LGUI) => {$crate::rpt!(@kb_rpt LeftGUI)};  (LSFT) => {$crate::rpt!(@kb_rpt LeftShift)};
    (RALT) => {$crate::rpt!(@kb_rpt RightAlt)}; (RCTL) => {$crate::rpt!(@kb_rpt RightControl)}; (RGUI) => {$crate::rpt!(@kb_rpt RightGUI)}; (RSFT) => {$crate::rpt!(@kb_rpt RightShift)};

    (P0) => {$crate::rpt!(@kb_rpt Keypad0)}; (P1) => {$crate::rpt!(@kb_rpt Keypad1)}; (P2) => {$crate::rpt!(@kb_rpt Keypad2)}; (P3) => {$crate::rpt!(@kb_rpt Keypad3)};
    (P4) => {$crate::rpt!(@kb_rpt Keypad4)}; (P5) => {$crate::rpt!(@kb_rpt Keypad5)}; (P6) => {$crate::rpt!(@kb_rpt Keypad6)}; (P7) => {$crate::rpt!(@kb_rpt Keypad7)};
    (P8) => {$crate::rpt!(@kb_rpt Keypad8)}; (P9) => {$crate::rpt!(@kb_rpt Keypad9)};

    (APP)  => {$crate::rpt!(@kb_rpt Application)};           (BSPC) => {$crate::rpt!(@kb_rpt DeleteBackspace)}; (BSLS) => {$crate::rpt!(@kb_rpt Backslash)};
    (CAPS) => {$crate::rpt!(@kb_rpt CapsLock)};              (COMM) => {$crate::rpt!(@kb_rpt Comma)};           (DEL)  => {$crate::rpt!(@kb_rpt DeleteForward)};
    (DOT)  => {$crate::rpt!(@kb_rpt Dot)};                   (DOWN) => {$crate::rpt!(@kb_rpt DownArrow)};       (END)  => {$crate::rpt!(@kb_rpt End)};
    (ENT)  => {$crate::rpt!(@kb_rpt ReturnEnter)};           (EQL)  => {$crate::rpt!(@kb_rpt Equal)};           (ESC)  => {$crate::rpt!(@kb_rpt Escape)};
    (GRV)  => {$crate::rpt!(@kb_rpt Grave)};                 (HOME) => {$crate::rpt!(@kb_rpt Home)};            (INS)  => {$crate::rpt!(@kb_rpt Insert)};
    (LBRC) => {$crate::rpt!(@kb_rpt LeftBrace)};             (LEFT) => {$crate::rpt!(@kb_rpt LeftArrow)};       (MINS) => {$crate::rpt!(@kb_rpt Minus)};
    (NO)   => {$crate::rpt!(@kb_rpt NoEventIndicated)};      (NUBS) => {$crate::rpt!(@kb_rpt NonUSBackslash)};  (NUHS) => {$crate::rpt!(@kb_rpt NonUSHash)};
    (NUM)  => {$crate::rpt!(@kb_rpt KeypadNumLockAndClear)}; (PAST) => {$crate::rpt!(@kb_rpt KeypadMultiply)};  (PAUS) => {$crate::rpt!(@kb_rpt Pause)};
    (PDOT) => {$crate::rpt!(@kb_rpt KeypadDot)};             (PEQL) => {$crate::rpt!(@kb_rpt KeypadEqual)};     (PENT) => {$crate::rpt!(@kb_rpt KeypadEnter)};
    (PGDN) => {$crate::rpt!(@kb_rpt PageDown)};              (PGUP) => {$crate::rpt!(@kb_rpt PageUp)};          (PPLS) => {$crate::rpt!(@kb_rpt KeypadAdd)};
    (PMNS) => {$crate::rpt!(@kb_rpt KeypadSubtract)};        (PWOR) => {$crate::rpt!(@kb_rpt Power)};           (PSCR) => {$crate::rpt!(@kb_rpt PrintScreen)};
    (PSLS) => {$crate::rpt!(@kb_rpt KeypadDivide)};          (QUOT) => {$crate::rpt!(@kb_rpt Apostrophe)};      (RBRC) => {$crate::rpt!(@kb_rpt RightBrace)};
    (RGHT) => {$crate::rpt!(@kb_rpt RightArrow)};            (SCLN) => {$crate::rpt!(@kb_rpt Semicolon)};       (SCRL) => {$crate::rpt!(@kb_rpt ScrollLock)};
    (SLSH) => {$crate::rpt!(@kb_rpt ForwardSlash)};          (SPC)  => {$crate::rpt!(@kb_rpt Space)};           (TAB)  => {$crate::rpt!(@kb_rpt Tab)};
    (UP)   => {$crate::rpt!(@kb_rpt UpArrow)};

    // Desktop
    (PWR)  => {$crate::rpt!(@dt_rpt SystemPowerDown)}; (SLEP) => {$crate::rpt!(@dt_rpt SystemSleep)}; (WAKE) => {$crate::rpt!(@dt_rpt SystemWakeUp)};

    // Consumer
    (MUTE) => {$crate::rpt!(@co_rpt Mute)};              (VOLU) => {$crate::rpt!(@co_rpt VolumeIncrement)};   (VOLD) => {$crate::rpt!(@co_rpt VolumeDecrement)};
    (MNXT) => {$crate::rpt!(@co_rpt TrackingIncrement)}; (MPRV) => {$crate::rpt!(@co_rpt TrackingDecrement)}; (MSTP) => {$crate::rpt!(@co_rpt Stop)};
    (MPLY) => {$crate::rpt!(@co_rpt PlayPause)};

    // Custom
    ($n: literal) => {
        {
            const _ : () = assert!($n >= 10 && $n <= 255, "Literal must be between 10 and 255");
            $crate::rpt!(@cu_rpt $n)
        }
    };
}
