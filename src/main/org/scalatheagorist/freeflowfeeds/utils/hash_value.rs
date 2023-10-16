use std::collections::hash_map::DefaultHasher;
use core::hash::{Hash, Hasher};
use serde::Serialize;

pub fn hash_value<R>(value: R) -> u64
    where R: Serialize + Hash {
    let serialized_value: String = serde_json::to_string(&value).unwrap();
    let mut hasher: DefaultHasher = DefaultHasher::new();
    serialized_value.hash(&mut hasher);

    hasher.finish()
}
