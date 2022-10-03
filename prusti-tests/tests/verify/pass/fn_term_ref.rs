// compile-flags: -Punsafe_core_proof=true

use prusti_contracts::*;

fn main() {}

fn foo(x: i64) {
    if x > 0 {
        foo(x - 1);
    }
}
