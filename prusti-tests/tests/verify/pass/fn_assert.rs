// compile-flags: -Punsafe_core_proof=true

use prusti_contracts::*;

#[pure]
#[terminates]
fn answer() -> i32 {
    42
}

fn test1() {
    prusti_assert!(answer() == 42);
    prusti_assert!(1 + 2 + 3 + 4 + 5 + 6 == 21);
}

fn main() {}
