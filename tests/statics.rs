use rukeeb::key_handler::chord::Chord;
use rukeeb::key_handler::{Process, Processor};
use rukeeb::*;

pub const N: usize = 6;

// layer 0
static HT: switch_handler::holdtap::HoldTap = ht!(12, k!(F), k!(J));
static PRO0: Processor<N> = processor!([
    kh!(A), HT;
    kh!(A), lo!(1, 0);
    kh!(A), lt!(1);]);
static CHORD1: Chord = chrd!((1, 2), &kh!(Q));
static CHORD2: Chord = chrd!((2, 4), &lt!(1));
static L1: [&'static dyn Process<N>; 3] = [&CHORD1, &CHORD2, &PRO0];

// layer 1
static PRO1: Processor<N> = processor!([
    kh!(B), kh!(B);
    kh!(B), kh!(B);
    lt!(2), lt!(0);
]);
static L2: [&'static dyn Process<N>; 1] = [&PRO1];

// layer 2
static PRO2: Processor<N> = processor!([
    kh!(C), kh!(A,B,C);
    kh!(C), kh!(C);
    kh!(C), lt!(0);
]);
static L3: [&'static dyn Process<N>; 1] = [&PRO2];

pub static HANDLERS: [&'static [&'static dyn Process<N>]; 3] = [&L1, &L2, &L3];
