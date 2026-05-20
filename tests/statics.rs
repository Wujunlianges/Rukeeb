use rukeeb::key_handler::{Chord, KeyHandle, Keymap};
use rukeeb::*;

pub const N: usize = 6;

static CHORD: Chord = chrd!([(0, 1, 2, kh!(Q)), (0, 2, 4, lt!(1))]);
static KEYMAP: Keymap<N> = keymap!(
    [
        [
            kh!(A), ht!(7, k!(F), k!(J));
            kh!(A), lo!(1, 0);
            kh!(A), lt!(1);
        ],
        [
            kh!(B), kh!(B);
            kh!(B), kh!(B);
            lt!(2), lt!(0);
        ],
        [
            kh!(C), kh!(A,B,C);
            kh!(C), kh!(C);
            kh!(C), lt!(0);
        ]
    ]
);

pub static KEY_HANDLERS: [&'static dyn KeyHandle<N>; 2] = [&CHORD, &KEYMAP];
