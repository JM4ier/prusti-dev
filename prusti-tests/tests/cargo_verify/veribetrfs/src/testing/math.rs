use prusti_contracts::*;

#[requires(x < 100)]
fn only_below_100(x: u64) {}

#[derive(Copy, Clone)]
struct S {
    a: u32,
    b: u64,
}

#[pure]
fn pure_returns_struct() -> S {
    S {a: 0, b: 1}
}