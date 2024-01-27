use core::hash::{Hash, Hasher};
use serde::Serialize;
use std::collections::hash_map::DefaultHasher;

pub fn hash_value<R>(value: &R) -> Option<u64>
    where
        R: Serialize + Hash,
{
    serde_json::to_string(&value).ok().map(|serialized| {
        let mut hasher: DefaultHasher = DefaultHasher::new();
        serialized.hash(&mut hasher);

        hasher.finish()
    })
}
