use rukeeb::processor::chord::Chord;
use rukeeb::processor::{Process, Processor};
use rukeeb::*;

pub const N: usize = 6;
pub const L: usize = 3;

// layer 0
static HT: handler::holdtap::HoldTap = ht!(12, k!(F), k!(J));
static PRO0: Processor<N> = processor!([
    kc!(A), HT;
    kc!(A), lo!(1, 0);
    kc!(A), lt!(1);]);
static CHORD1: Chord = chrd!((1, 2), &kc!(Q));
static CHORD2: Chord = chrd!((2, 4), &lt!(1));
static L1: [&'static dyn Process<N>; 3] = [&CHORD1, &CHORD2, &PRO0];

// layer 1
static PRO1: Processor<N> = processor!([
    kc!(B), kc!(B);
    kc!(B), kc!(B);
    lt!(2), lt!(0);
]);
static CHORD3: Chord = chrd!((2, 4), &lt!(0));
static L2: [&'static dyn Process<N>; 2] = [&CHORD3, &PRO1];

// layer 2
static PRO2: Processor<N> = processor!([
    kc!(C), kc!(C);
    kc!(C), kc!(C);
    kc!(C), lt!(0);
]);
static L3: [&'static dyn Process<N>; 1] = [&PRO2];

pub static HANDLERS: [&'static [&'static dyn Process<N>]; 3] = [&L1, &L2, &L3];
