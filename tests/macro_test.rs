mod test {
    use rukeeb::report::Report;
    use rukeeb::rpt;

    macro_rules! test_rpt {
        ($($x:tt),* $(,)?) => {
            [$(rpt!($x),)*]
        };
    }

    #[test]
    fn test_report() {
        #[rustfmt::skip]
        let _reports: [Report; 131] = test_rpt![
            // Keyboard
            A, B, C, D, E, F, G, H, I, J, K, L, M,
            N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
            F1,  F2,  F3,  F4,  F5,  F6,  F7,  F8,  F9,  F10, F11, F12,
            F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24,
            LALT, LCTL, LGUI, LSFT, RALT, RCTL, RGUI, RSFT,
            P0, P1, P2, P3, P4, P5, P6, P7, P8, P9,
            APP,  BSPC, BSLS, CAPS, COMM, DEL,  DOT,  DOWN, END,  ENT,  EQL,  ESC,
            GRV,  HOME, INS,  LBRC, LEFT, MINS, NO,   NUBS, NUHS, NUM,  PAST, PAUS,
            PDOT, PEQL, PENT, PGDN, PGUP, PPLS, PMNS, PWOR, PSCR, PSLS, QUOT,
            RBRC, RGHT, SCLN, SCRL, SLSH, SPC,  TAB,  UP,

            // Desktop
            PWR, SLEP, WAKE,

            // Consumer
            MUTE, VOLU, VOLD, MNXT, MPRV, MSTP, MPLY,

            // Customer
            // todo
        ];
    }
}
