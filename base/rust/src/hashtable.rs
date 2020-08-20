
extern crate uuid;
extern crate rand;

use std::env;
use std::collections::HashMap;
use uuid::Uuid;
use rand::Rng;

pub fn do_compute() {
    let mut rng = rand::thread_rng();

    let size: usize = match env::var_os("HASHTABLE_SIZE") {
        Some(val) => val.into_string().unwrap().parse().unwrap_or(0),
        None => panic!("HASHTABLE_SIZE is not defined in the environment")
    };
    let read_count: usize = match env::var_os("READ_COUNT") {
        Some(val) => val.into_string().unwrap().parse().unwrap_or(0),
        None => panic!("READ_COUNT is not defined in the environment")
    };

    // create a hashtable and an input vector (Variable-length array not supported in Rust)
    let mut hashtable = HashMap::new();
    let mut keys: Vec<uuid::Uuid> = Vec::with_capacity(size);

    // init the hash table
    for _i in 0..size {
        let key = Uuid::new_v4();
        hashtable.insert(key, key);
        keys.push(key);
    }

    //access hash table
    for _i in 0..read_count {
        let index = rng.gen_range(0, size);
        let key = keys[index];
        hashtable[&key];
    }
}
