// compile-flags: -Punsafe_core_proof=true

#![allow(unused)]

use prusti_contracts::*;

fn main() {
    let mut x = 10;
    while x > 0 {
        body_variant!(Int::new(x));
        x -= 1;
    }
}
