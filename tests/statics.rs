use rukeeb::key_handler::{Chord, Combo, HandleKeyEvent, Keymap};
use rukeeb::*;

pub const N: usize = 6;

static COMBO: Combo = combo!([(2, 3, [kh!(A), kh!(B), kh!(C)]),]);
static CHORD: Chord = chord!([(0, 1, 2, kh!(Q)), (0, 2, 4, lt!(1))]);
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
            kh!(C), kh!(Z); // this is overridden by combo
            kh!(C), kh!(C);
            kh!(C), lt!(0);
        ]
    ]
);

pub static KEY_HANDLERS: [&'static dyn HandleKeyEvent<N>; 3] = [&COMBO, &CHORD, &KEYMAP];
