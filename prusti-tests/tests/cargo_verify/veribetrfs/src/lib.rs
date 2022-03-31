// <-- consistency error reported on first line

use prusti_contracts::*;

struct Vector<T>(T);
struct Wrapper<T>(T);

impl<T> Vector<T> {
    #[pure]
    fn len(&self) -> usize {1}

    #[requires(idx < self.len())]
    fn index(&self, idx: usize) {}
}

fn first<T>(vec: &Vector<Wrapper<T>>) {
    vec.index(0)
}
