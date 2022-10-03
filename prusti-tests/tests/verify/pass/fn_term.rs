// compile-flags: -Punsafe_core_proof=true

use prusti_contracts::*;

fn main() {}

#[terminates(Int::new(x) + Int::new(1))]
fn foo(x: i64) {
    bar(x);
    if x > 0 {
        foo(x - 1);
    }
}

#[terminates(Int::new(x))]
fn bar(x: i64) {
    if x > 0 {
        bar(x - 1);
    }
}
