// compile-flags: -Punsafe_core_proof=true

use prusti_contracts::*;

fn test1() {
    let mut a = [1; 100];
    a[1] = 2;
    assert!(a[1] == 2);
    assert!(a[0] == 1);
}

fn main() {}
