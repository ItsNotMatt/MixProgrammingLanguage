use std::{collections::{HashMap, hash_map::DefaultHasher}, hash::{Hash, Hasher}};

pub fn get_hash(identifier: &String) -> u64 {
    let mut s = DefaultHasher::new();
    identifier.hash(&mut s);
    s.finish()
}

pub fn validate_len(args_size: usize, required_size: usize) {
    if args_size != required_size {
        panic!("Function expected: {} args, but only {} were passed in", args_size, required_size);
    }
}

