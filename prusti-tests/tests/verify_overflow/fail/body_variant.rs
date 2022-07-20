// compile-flags: -Punsafe_core_proof=true

#![allow(unused)]

use prusti_contracts::*;

#[terminates]
fn main() {
    while false {
        body_invariant!(false);
        body_variant!(Int::new(0));
    }
}
