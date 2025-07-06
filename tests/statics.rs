use rukeeb::processor::chord::Chord;
use rukeeb::processor::{Process, Processor};
use rukeeb::*;

pub const N: usize = 6;
pub const L: usize = 3;

// layer 0
static TAP_COMB: handler::tapcomb::TapComb = tc!([kb!(W), kb!(E)]);
static HOLD_TAP: handler::holdtap::HoldTap = ht!(12, kb!(F), kb!(J));
static PRO0: Processor<N> = processor!([
    kc!(A), HOLD_TAP;
    kc!(A), lyoo!(1, 0);
    TAP_COMB, lytp!(1);]);
static CHORD1: Chord = chrd!((1, 2), &kc!(Q));
static CHORD2: Chord = chrd!((2, 4), &lytp!(1));
static L1: [&'static dyn Process<N>; 3] = [&CHORD1, &CHORD2, &PRO0];

// layer 1
static PRO1: Processor<N> = processor!([
    kc!(B), kc!(B);
    kc!(B), kc!(B);
    lytp!(2), lytp!(0);
]);
static CHORD3: Chord = chrd!((2, 4), &lytp!(0));
static L2: [&'static dyn Process<N>; 2] = [&CHORD3, &PRO1];

// layer 2
static PRO2: Processor<N> = processor!([
    kc!(C), kc!(C);
    kc!(C), kc!(C);
    kc!(C), lytp!(0);
]);
static L3: [&'static dyn Process<N>; 1] = [&PRO2];

pub static HANDLERS: [&'static [&'static dyn Process<N>]; 3] = [&L1, &L2, &L3];
