
use std::env;

pub fn do_compute() {
    // size of the array to sort
    let size: usize = match env::var_os("ARRAY_SIZE") {
        Some(val) => val.into_string().unwrap().parse().unwrap_or(0),
        None => panic!("ARRAY_SIZE is not defined in the environment")
    };

    //create an input array with unsorted values
    let mut vec: Vec<i32> = Vec::with_capacity(size);
    let mut incr: i32 = 1;
    vec.push(0);
    for i in 1..size {
        vec.push(-vec[i-1] + incr);
        incr = -incr;
    }

    //sort the array
    vec.sort();
}
