use prusti_contracts::*;

fn while1() {
    let mut i = 0;
    while i < 10 {
        body_invariant!(i < 11);
        // [Prusti: verification error] the asserted expression might not hold
        // error with version v-2022-03-24-1221, dacab2d43007ed6eeb88d87d2a04b061f8684f91
        assert!(i < 10);
        i += 1;
    }
}

fn while2() {
    let mut i = 0;
    while i < 10 {
        body_invariant!(i < 11);
        i += 1;
    }
}