use crate::base::*;
use super::fixed_size::*;
use super::base::*;

pub struct HashMap<V> {
    inner: FixedSizeHashMap<V>,
    count: u64,
}

impl<V: Clone> HashMap<V> {
    #[requires(128 <= size)]
    pub fn with_size(size: u64) -> Self {
        Self {
            inner: FixedSizeHashMap::with_size(size),
            count: 0,
        }
    }

    pub fn realloc(&mut self) {
        let new_size = (128 + self.count) * 4;
        let mut new_inner = FixedSizeHashMap::with_size(new_size);

        let mut i = 0;
        while i < self.inner.storage.len() {
            let item = self.inner.storage.index(i).clone();
            if let Entry { key, value } = item {
                new_inner.insert(key, value);
            }
            i += 1;
        }

        self.inner = new_inner;
    }

    pub fn insert(&mut self, key: u64, value: V) -> Opt<V> {
        if self.inner.storage.len() / 2 < self.inner.count {
            self.realloc();
        }
        let replaced = self.inner.insert(key, value);
        if replaced.is_none() {
            self.count += 1;
        }
        replaced
    }

    pub fn remove(&mut self, key: u64) -> Opt<V> {
        let removed = self.inner.remove(key);
        if removed.is_some() {
            self.count -= 1;
        }
        removed
    }

    pub fn get(&self, key: u64) -> Opt<V> {
        self.inner.get(key)
    }
}