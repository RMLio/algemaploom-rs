//! Some utility functions

use std::collections::HashMap;
use std::hash::{Hash, Hasher};

/// Hashes entries of a HashMap in a deterministic way by sorting the entries by key before hashing.
pub fn hash_hashmap<H, K, V>(hash_map: &HashMap<K, V>, state: &mut H)
where
    H: Hasher,
    K: Hash + Ord,
    V: Hash,
{
    let mut pairs: Vec<_> = hash_map.iter().collect();
    pairs.sort_by(|pair1, pair2| pair1.0.cmp(pair2.0));
    for (key, value) in pairs {
        key.hash(state);
        value.hash(state);
    }
}