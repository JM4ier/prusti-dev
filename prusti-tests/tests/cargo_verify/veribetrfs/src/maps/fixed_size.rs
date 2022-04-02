use crate::base::*;
use super::base::*;

pub struct FixedSizeHashMap<V> {
    pub storage: Vector<Item<V>>,
    pub count: u64,
}

impl<V: Clone + Eq> FixedSizeHashMap<V> {
    #[pure]
    fn inv(&self) -> bool {
        128 <= self.storage.len() && self.count <= self.storage.len()
    }

    #[requires(128 <= size)]
    #[ensures(result.inv())]
    pub fn with_size(size: u64) -> Self {
        let storage = Vector::init(Empty, size);
        Self { storage, count: 0 }
    }

    #[requires(128 <= storage.len())]
    #[ensures(result.inv())]
    pub fn from_storage(storage: Vector<Item<V>>) -> Self {
        let count = Self::count_entries(0, storage);
        unreachable!();
        Self { storage, count }
    }

    #[requires(idx <= storage.len())]
    #[ensures(idx + result <= storage.len())]
    fn count_entries(idx: u64, storage: Vector<Item<V>>) -> u64 {
        if idx == storage.len() {
            0
        } else {
            let c = if storage.index(idx).is_entry() {1} else {0};
            c + Self::count_entries(idx + 1, storage)
        }
    }

    #[requires(self.inv())]
    #[pure]
    pub fn capacity(&self) -> u64 {
        self.storage.len()
    }

    #[requires(self.inv())]
    #[ensures(self.inv())]
    #[ensures(result < self.capacity())]
    pub fn slot_for_key(&self, key: u64) -> u64 {
        let h = hash64(key);
        let len = self.storage.len();
        h % len
    }

    #[requires(self.inv())]
    #[ensures(self.inv())]
    #[ensures(result < self.storage.len())]
    fn probe(&self, key: u64) -> u64 {
        let mut slot_idx = self.slot_for_key(key);
        let mut done = false;

        while !done {
            body_invariant!(self.inv());
            body_invariant!(slot_idx < self.storage.len());
            let k = key;
            match self.storage.index(slot_idx) {
                Empty => done = true,
                Tombstone { key } => {
                    done = *key == k;
                }
                Entry { key, .. } => {
                    done = *key == k;
                }
                _ => (),
            };
            slot_idx = slot_succ(self.storage.len(), slot_idx);
        }

        slot_idx
    }

    #[requires(self.inv())]
    #[ensures(self.inv())]
    pub fn insert(&mut self, key: u64, value: V) -> Opt<V> {
        let mut slot_idx = self.probe(key);
        let slot = self.storage.index_mut(slot_idx);
        let original = replace(slot, Entry { key, value });
        match original {
            Entry { key: _, value } => Opt::Some(value),
            _ => {
                self.count += 1;
                Opt::None
            }
        }
    }

    #[requires(slot_idx < self.storage.len())]
    #[requires(self.inv())]
    #[ensures(self.inv())]
    fn update_slot(&mut self, slot_idx: u64, value: V) {
        let slot = self.storage.index_mut(slot_idx);
        match slot {
            Entry { key: _, value: val } => {
                *val = value;
            }
            _ => (),
        }
    }

    #[requires(self.inv())]
    #[ensures(self.inv())]
    pub fn remove(&mut self, key: u64) -> Opt<V> {
        let slot_idx = self.probe(key);
        let slot = self.storage.index_mut(slot_idx);
        if (&*slot).is_entry() {
            let old = replace(slot, Tombstone{key});
            match old {
                Entry { key: _, value } => {
                    if self.count > 0 {
                        // should always be the case
                        self.count -= 1;
                    }
                    Opt::Some(value)
                }
                _ => unreachable!(),
            }
        } else {
            Opt::None
        }
    }

    #[requires(self.inv())]
    #[ensures(self.inv())]
    pub fn get(&self, key: u64) -> Opt<V> {
        let slot_idx = self.probe(key);
        let slot = self.storage.index(slot_idx);
        match slot {
            Entry { key: _, value: val } => Opt::Some(val.clone()),
            _ => Opt::None,
        }
    }
}
