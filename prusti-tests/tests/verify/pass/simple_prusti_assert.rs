// compile-flags: -Punsafe_core_proof=true

use prusti_contracts::*;

fn main() {
    prusti_assert!(42 < 43);
}
