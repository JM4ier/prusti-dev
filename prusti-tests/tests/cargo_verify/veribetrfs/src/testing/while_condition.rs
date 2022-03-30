use prusti_contracts::*;
use crate::base::*;

fn while1() {
    let mut i = 0;
    while i < 10 {
        // [Prusti: verification error] the asserted expression might not hold
        // error with version v-2022-03-24-1221, dacab2d43007ed6eeb88d87d2a04b061f8684f91
        assert!(i < 10);
        i += 1;
    }
}

fn while2() {
    let mut i = 0;
    while i < 10 {
        body_invariant!(i < 10);
        assert!(i < 10);
        i += 1;
    }
}

#[ensures(result.1 <= old(vec.len()))]
#[ensures(result.0 == old(vec))]
fn count_div2(vec: Vector<u32>) -> (Vector<u32>, u64) {
    let mut count = 0;
    let mut i = 0;
    let two = 2;
    while i < vec.len() {
        body_invariant!(i < vec.len());
        body_invariant!(count <= i);
        if vec.index(i) % two == 0 {
            count += 1;
        }
        i += 1;
    }
    (vec, count)
}