// compile-flags: -Punsafe_core_proof=true

use prusti_contracts::*;

fn main() {
    let mut x: i32 = 42;
    x = 43;
    x = 44;
}
