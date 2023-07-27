use std::{collections::{HashMap, hash_map::DefaultHasher}, hash::{Hash, Hasher}};

pub fn get_hash(identifier: &String) -> u64 {
    let mut s = DefaultHasher::new();
    identifier.hash(&mut s);
    s.finish()
}
