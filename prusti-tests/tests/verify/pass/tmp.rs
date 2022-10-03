// compile-flags: -Punsafe_core_proof=true

#![allow(unused)]
use prusti_contracts::*;
fn main() {}


pub fn replace<T>(dest: &mut T, src: T) -> T {
    std::mem::replace(dest, src)
}

#[requires(len > 0)]
#[requires(slot < len)]
pub fn slot_succ(len: u64, slot: u64) -> u64 {
    if slot == len - 1 {
        0
    } else {
        slot + 1
    }
}

#[pure]
#[trusted]
fn add(a: u64, b: u64) -> u64 {
    a.wrapping_add(b)
}

#[pure]
#[trusted]
pub fn hash64(k: u64) -> u64 {
    let k0 = add(!k, k << 21);
    let k1 = k0 ^ (k0 >> 24);
    let k2 = add(k1, add(k1 << 3, k1 << 8));
    let k3 = k2 ^ (k2 >> 14);
    let k4 = add(k3, add(k3 << 2, k3 << 4));
    let k5 = k4 ^ (k4 >> 28);
    let k6 = add(k5, k5 << 31);
    k6
}

pub struct Vector<T>(Vec<T>);
impl<T> Vector<T> {
    #[trusted]
    #[ensures(result.len() == 0)]
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn from_vec(vec: Vec<T>) -> Self {
        Self(vec)
    }

    #[trusted]
    #[pure]
    pub fn len(&self) -> u64 {
        self.0.len() as _
    }

    #[trusted]
    #[requires(idx < self.len())]
    #[pure]
    pub fn index(&self, idx: u64) -> &T {
        &self.0[idx as usize]
    }

    #[trusted]
    #[requires(idx < self.len())]
    #[after_expiry(self.len() == old(self.len()))]
    pub fn index_mut(&mut self, idx: u64) -> &mut T {
        &mut self.0[idx as usize]
    }
}
impl<T: Clone> Vector<T> {
    #[trusted]
    #[ensures(result.len() == len)]
    pub fn init(init: T, len: u64) -> Self {
        Self(vec![init; len as usize])
    }
}

pub enum Opt<T> {
    None,
    Some(T),
}
use Opt::*;
impl<T> Opt<T> {
    #[pure]
    #[trusted]
    pub fn is_none(&self) -> bool {
        matches!(self, None)
    }

    #[pure]
    #[trusted]
    pub fn is_some(&self) -> bool {
        matches!(self, Some(..))
    }

    #[requires(self.is_some())]
    pub fn unwrap(self) -> T {
        match self {
            Some(t) => t,
            None => panic!("called unwrap on `None` instance."),
        }
    }

    #[ensures(result.is_some())]
    pub fn wrap(t: T) -> Self {
        Some(t)
    }
}

#[derive(Clone)]
pub enum Item<V> {
    Empty,
    Entry { key: u64, value: V },
    Tombstone { key: u64 },
}
pub use Item::{Empty, Entry, Tombstone};
impl<V> Item<V> {
    #[pure]
    pub fn is_empty(&self) -> bool {
        matches!(self, Empty)
    }
    #[pure]
    pub fn is_entry(&self) -> bool {
        matches!(self, Entry { .. })
    }
    #[pure]
    pub fn is_tombstone(&self) -> bool {
        matches!(self, Tombstone { .. })
    }
}

pub struct FixedSizeLinearHashMap<V> {
    storage: Vector<Item<V>>,
    count: u64,
}

impl<V: Clone> FixedSizeLinearHashMap<V> {
    #[pure]
    fn inv(&self) -> bool {
        128 <= self.storage.len() && self.count < self.storage.len()
    }

    #[requires(128 <= size)]
    #[ensures(result.inv())]
    pub fn with_size(size: u64) -> Self {
        let storage = Vector::init(Empty, size);
        Self { storage, count: 0 }
    }

    #[requires(self.inv())]
    #[ensures(result < self.storage.len())]
    pub fn slot_for_key(&self, key: u64) -> u64 {
        let h = hash64(key);
        let len = self.storage.len();
        h % len
    }

    #[requires(self.inv())]
    #[requires(forall(|k: u64| k < i ==> !self.storage.index(i).is_empty()))]
    #[requires(i < self.storage.len())]
    pub fn get_empty_witness(&self, i: u64) -> u64 {
        let entry = self.storage.index(i);
        match entry {
            Empty => i,
            _ => self.get_empty_witness(i + 1),
        }
    }

    #[requires(self.inv())]
    #[ensures(result < self.storage.len())]
    pub fn probe(&self, key: u64) -> u64 {
        let mut slot_idx = self.slot_for_key(key);
        let start_slot_idx = slot_idx;
        let mut done = false;

        while !done {
            body_invariant!(self.inv());
            let k = key;
            match self.storage.index(slot_idx) {
                Empty => done = true,
                Tombstone { key } => {
                    if *key == k {
                        break;
                    }
                }
                Entry { key, .. } => {
                    if *key == k {
                        break;
                    }
                }
                _ => (),
            };
            slot_idx = slot_succ(self.storage.len(), slot_idx);
        }

        slot_idx
    }

    #[requires(self.inv())]
    #[ensures(self.inv())]
    #[trusted]
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

    #[requires(self.inv())]
    #[ensures(self.inv())]
    #[requires(slot_idx < self.storage.len())]
    pub fn update_slot(&mut self, slot_idx: u64, value: V) {
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
        if slot.is_entry() {
            let old = replace(slot, Tombstone { key });
            match old {
                Entry { key: _, value } => {
                    self.count -= 1;
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
