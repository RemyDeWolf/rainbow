
extern crate uuid;
extern crate rand;

use std::env;
use rand::Rng;
use std::fs;

pub fn do_compute() {
    let mut rng = rand::thread_rng();

    let writes: usize = match env::var_os("WRITES") {
        Some(val) => val.into_string().unwrap().parse().unwrap_or(0),
        None => panic!("WRITES is not defined in the environment")
    };
    let reads: usize = match env::var_os("READS") {
        Some(val) => val.into_string().unwrap().parse().unwrap_or(0),
        None => panic!("READS is not defined in the environment")
    };
    let file_size: usize = match env::var_os("FILE_SIZE") {
        Some(val) => val.into_string().unwrap().parse().unwrap_or(0),
        None => panic!("FILE_SIZE is not defined in the environment")
    };

    // create the file data
    let data: String = "0".repeat(file_size);

    // write the files
    for i in 0..writes {
        let path = format!("/data/{}", i);
        fs::write(path.clone(), data.clone()).expect("Unable to write file");
    }

    // read the files
    for _i in 0..reads {
        let index = rng.gen_range(0, writes);
        let path = format!("/data/{}", index);
        fs::read(path.clone()).expect("Unable to read file");
    }

}
