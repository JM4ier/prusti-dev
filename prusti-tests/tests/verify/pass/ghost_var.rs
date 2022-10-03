// compile-flags: -Punsafe_core_proof=true

use prusti_contracts::*;

fn main() {
    let mut x: Ghost<i32> = Ghost::new(42);
    x = Ghost::new(43);
    x = Ghost::new(44);
}
