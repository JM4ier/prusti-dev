// compile-flags: -Punsafe_core_proof=true

use prusti_contracts::*;

fn main() {
    let mut i = 0;
    i += 1;
    i += 1;
    i += 1;
}
