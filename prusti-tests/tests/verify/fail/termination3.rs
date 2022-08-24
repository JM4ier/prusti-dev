// compile-flags: -Punsafe_core_proof=true

#![allow(unused)]

use prusti_contracts::*;

fn main() {
    let mut z = 10;
    while z > 1 {
        body_invariant!(z > 1);
        body_variant!(Int::new(z));
        z -= 1;
    }
}
