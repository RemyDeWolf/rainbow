
mod compute;
extern crate redis;
extern crate uuid;
extern crate rand;

use std::env;
use compute::do_compute;
use log::Level;

use std::sync::{Arc, Barrier};
use std::thread;

#[macro_use]
extern crate log;
extern crate simple_logger;

fn main() {
    simple_logger::init_with_level(Level::Info).unwrap();

    let workers: usize = match env::var_os("WORKERS") {
        Some(val) => val.into_string().unwrap().parse().unwrap_or(0),
        None => 1
    };
    info!("workers: {}", workers);

    let mut handles = Vec::with_capacity(workers);
    let barrier = Arc::new(Barrier::new(workers));
    for n in 0..workers {
        let c = barrier.clone();
        handles.push(thread::spawn(move|| {
            run_compute(n);
            c.wait();
        }));
    }

    // Wait for other threads to finish.
    for handle in handles {
        handle.join().unwrap();
    }
}

fn run_compute(n: usize) {
    info!("Starting worker {}", n);

    let compute = match env::var_os("COMPUTE") {
        Some(val) => val.into_string().unwrap(),
        None => panic!("COMPUTE is not defined in the environment")
    };
    let key = format!("rust-{}.computed", compute);
    info!("Redis key: {}", key);

    let client = redis::Client::open("redis://@redis:6379").unwrap();
    let mut conn = client.get_connection().unwrap();

    let max_compute: i32 = env::var("MAX_COMPUTE").unwrap_or("0".to_string()).parse().unwrap();
    let batch_size: i32 = env::var("BATCH_SIZE").unwrap_or("1".to_string()).parse().unwrap();

    let mut count = 0;
    loop {
        do_compute();
        count=count+1;
        if count%batch_size == 0 {
            redis::cmd("INCRBY").arg(&key).arg(batch_size.to_string()).execute(&mut conn);
            info!("[{}] Computed {} operations", n, count);
        }
        if max_compute!=0 && count>=max_compute {
            break;
        }
    }
}
