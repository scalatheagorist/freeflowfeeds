use core::hash::{Hash, Hasher};
use serde::Serialize;
use std::collections::hash_map::DefaultHasher;

pub fn hash_value<R>(value: &R) -> Option<u64>
where
    R: Serialize + Hash,
{
    match serde_json::to_string(&value) {
        Ok(serialized_value) => {
            let mut hasher: DefaultHasher = DefaultHasher::new();
            serialized_value.hash(&mut hasher);

            Some(hasher.finish())
        }
        Err(_) => None,
    }
}
