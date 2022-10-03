// compile-flags: -Punsafe_core_proof=true

#![allow(unused)]

use prusti_contracts::*;

fn main() {}


#[pure]
#[terminates(Int::new(n))]
#[requires(n >= 0)]
fn fib(n: i64) -> i64 {
    if n < 2 {
        n
    } else {
        fib(n-1) + fib(n-2)
    }
}


#[pure]
#[terminates(Int::new(n) - Int::new(i))]
#[requires(i >= 0 && i <= n)]
#[requires(f1 == fib(i) && f2 == fib(i + 1))]
#[ensures(result == fib(n))]
fn fast_aux(f1: i64, f2: i64, i: i64, n: i64) -> i64 {
    if i == n {
        f1
    } else {
        assert!(i < n);
        assert!(n - (i + 1) >= 0);
        assert!(n - (i + 1) < n - i);
        fast_aux(f2, f1 + f2, i + 1, n)
    }
}

#[pure]
#[terminates(Int::new(n))]
#[requires(n >= 0)]
#[ensures(result == fib(n))]
fn fast_fib(n: i64) -> i64 {
    fast_aux(0, 1, 0, n)
}

#[pure]
#[terminates(Int::new(n))]
#[requires(n >= 0)]
#[ensures((n % 3 == 0) == (fib(n) % 2 == 0))]
fn lemma(n: i64) {
    if n >= 3 {
        fib(n - 1);
        fib(n - 2);
        lemma(n - 1);
        lemma(n - 2);
    }
}
