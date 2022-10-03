// compile-flags: -Punsafe_core_proof=true

use prusti_contracts::*;

#[terminates]
fn main() {
    let mut i = 0;
    while i < 100 {
        body_variant!(Int::new(1000) - Int::new(i));
        i += 1;
    }
}
