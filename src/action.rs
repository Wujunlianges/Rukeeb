use crate::event::Event;
use crate::hid_report::HIDReport;

#[derive(Clone, Copy)]
pub enum KeyboardAction {
    HIDReport(HIDReport),
    LayerAction(LayerAction),
}

#[derive(Clone, Copy)]
pub enum KeyAction {
    HIDReport(HIDReport),
    LayerAction(LayerAction),
    CustomAction(&'static dyn CustomAction),
}

impl KeyAction {
    pub fn event(&self, event: &Event) -> Option<KeyboardAction> {
        match *self {
            KeyAction::HIDReport(hid_report) => match event {
                Event::Press(_) | Event::Pressed(_) => Some(KeyboardAction::HIDReport(hid_report)),
                _ => None,
            },

            KeyAction::LayerAction(layer_action) => match event {
                Event::Press(_) => match layer_action {
                    LayerAction::UndoCurrentLayer(_) => None,
                    _ => Some(KeyboardAction::LayerAction(layer_action)),
                },
                Event::Release(_) => match layer_action {
                    LayerAction::UndoCurrentLayer(_) => {
                        Some(KeyboardAction::LayerAction(layer_action))
                    }
                    _ => None,
                },
                _ => None,
            },

            KeyAction::CustomAction(custom_action) => custom_action.event(event),
        }
    }
}

#[derive(Clone, Copy)]
pub enum LayerAction {
    CurrentLayer(usize),
    UndoCurrentLayer(usize),
    DefaultLayer(usize),
}

pub trait CustomAction: Sync {
    fn event(&self, event: &Event) -> Option<KeyboardAction>;
}

#[macro_export]
macro_rules! kc {
    ($x: tt) => {{
        use crate::hid_report::HIDReport;
        use usbd_human_interface_device::page::Keyboard;
        KeyAction::HIDReport(HIDReport::Keyboard(Keyboard::$x))
    }};
}
pub(crate) use kc;

#[macro_export]
macro_rules! kbc {
    ($x: tt) => {{
        use crate::hid_report::HIDReport;
        use usbd_human_interface_device::page::Keyboard;
        KeyboardAction::HIDReport(HIDReport::Keyboard(Keyboard::$x))
    }};
}
pub(crate) use kbc;

#[macro_export]
macro_rules! kcl {
    ($x:literal) => {
        KeyAction::LayerAction(LayerAction::CurrentLayer($x))
    };
}
pub(crate) use kcl;

#[macro_export]
macro_rules! kbcl {
    ($x:literal) => {
        KeyboardAction::LayerAction(LayerAction::CurrentLayer($x))
    };
}
pub(crate) use kbcl;

#[macro_export]
macro_rules! kdl {
    ($x:literal) => {
        KeyAction::LayerAction(LayerAction::DefaultLayer($x))
    };
}
pub(crate) use kdl;

#[macro_export]
macro_rules! kbdl {
    ($x:literal) => {
        KeyboardAction::LayerAction(LayerAction::DefaultLayer($x))
    };
}
pub(crate) use kbdl;

#[macro_export]
macro_rules! kul {
    ($x:literal) => {
        KeyAction::LayerAction(LayerAction::UndoCurrentLayer($x))
    };
}
pub(crate) use kul;

#[macro_export]
macro_rules! kbul {
    ($x:literal) => {
        KeyboardAction::LayerAction(LayerAction::UndoCurrentLayer($x))
    };
}
pub(crate) use kbul;

// use usbd_human_interface_device::page::Keyboard as Keycode;

// macro_rules! gen_key_actions {
//     ($($new:ident : $old:ident)+) => {
//         $(
//             pub const $new: KeyAction = KeyAction::HIDReport(HIDReport::Keyboard(Keycode::$old));
//          )+
//     };
// }

// gen_key_actions! {
//     KC_NO   : NoEventIndicated
//     KC_A    : A
//     KC_B    : B
//     KC_C    : C
//     KC_D    : D
//     KC_E    : E
//     KC_F    : F
//     KC_G    : G
//     KC_H    : H
//     KC_I    : I
//     KC_J    : J
//     KC_K    : K
//     KC_L    : L
//     KC_M    : M
//     KC_N    : N
//     KC_O    : O
//     KC_P    : P
//     KC_Q    : Q
//     KC_R    : R
//     KC_S    : S
//     KC_T    : T
//     KC_U    : U
//     KC_V    : V
//     KC_W    : W
//     KC_X    : X
//     KC_Y    : Y
//     KC_Z    : Z
//     KC_1    : Keyboard1
//     KC_2    : Keyboard2
//     KC_3    : Keyboard3
//     KC_4    : Keyboard4
//     KC_5    : Keyboard5
//     KC_6    : Keyboard6
//     KC_7    : Keyboard7
//     KC_8    : Keyboard8
//     KC_9    : Keyboard9
//     KC_0    : Keyboard0
//     KC_ENT  : ReturnEnter
//     KC_ESC  : Escape
//     KC_BSPC : DeleteBackspace
//     KC_TAB  : Tab
//     KC_SPC  : Space
//     KC_MINS : Minus
//     KC_EQL  : Equal
//     KC_LBRC : LeftBrace
//     KC_RBRC : RightBrace
//     KC_BSLS : Backslash
//     KC_NUHS : NonUSHash
//     KC_SCLN : Semicolon
//     KC_QUOT : Apostrophe
//     KC_GRV  : Grave
//     KC_COMM : Comma
//     KC_DOT  : Dot
//     KC_SLSH : ForwardSlash
//     KC_CAPS : CapsLock
//     KC_F1   : F1
//     KC_F2   : F2
//     KC_F3   : F3
//     KC_F4   : F4
//     KC_F5   : F5
//     KC_F6   : F6
//     KC_F7   : F7
//     KC_F8   : F8
//     KC_F9   : F9
//     KC_F10  : F10
//     KC_F11  : F11
//     KC_F12  : F12
//     KC_PSCR : PrintScreen
//     KC_SCRL : ScrollLock
//     KC_PAUS : Pause
//     KC_INS  : Insert
//     KC_HOME : Home
//     KC_PGUP : PageUp
//     KC_DEL  : DeleteForward
//     KC_END  : End
//     KC_RGHT : RightArrow
//     KC_LEFT : LeftArrow
//     KC_DOWN : DownArrow
//     KC_UP   : UpArrow
//     KC_NUM  : KeypadNumLockAndClear
//     KC_PSLS : KeypadDivide
//     KC_PAST : KeypadMultiply
//     KC_PPLS : KeypadAdd
//     KC_PENT : KeypadEnter
//     KC_P1   : Keypad1
//     KC_P2   : Keypad2
//     KC_P3   : Keypad3
//     KC_P4   : Keypad4
//     KC_P5   : Keypad5
//     KC_P6   : Keypad6
//     KC_P7   : Keypad7
//     KC_P8   : Keypad8
//     KC_P9   : Keypad9
//     KC_P0   : Keypad0
//     KC_PDOT : KeypadDot
//     KC_NUBS : NonUSBackslash
//     KC_APP  : Application

//     KC_LCTL : LeftControl
//     KC_LSFT : LeftShift
//     KC_LALT : LeftAlt
//     KC_LGUI : LeftGUI
//     KC_RCTL : RightControl
//     KC_RSFT : RightShift
//     KC_RALT : RightAlt
//     KC_RGUI : RightGUI
// }
