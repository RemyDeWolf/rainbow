
use std::env;

pub fn do_compute() {
    // size of the array to compute
    let size: usize = match env::var_os("NSQUARE_ARRAY_SIZE") {
        Some(val) => val.into_string().unwrap().parse().unwrap_or(0),
        None => panic!("NSQUARE_ARRAY_SIZE is not defined in the environment")
    };

    // create an input vector (Variable-length array not supported in Rust)
    let mut vec: Vec<usize> = Vec::with_capacity(size);
    for _i in 0..size {
        vec.push(0);
    }

    for i in 0..size {
        for j in 0..size {
            vec[j] = (vec[i] + j) / 2
        }
    }
}
