use crate::base::*;

#[derive(PartialEq, Eq)]
pub struct Vector<T>(Vec<T>);

impl<T> Vector<T> {
    #[trusted]
    #[pure]
    pub fn len(&self) -> u64 {
        self.0.len() as _
    }

    #[trusted]
    #[requires(idx < self.len())]
    pub fn index(&self, idx: u64) -> &T {
        &self.0[idx as usize]
    }
}