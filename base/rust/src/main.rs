
mod compute;
extern crate redis;
extern crate uuid;
extern crate rand;
extern crate threadpool;

use std::env;
use compute::do_compute;
use log::Level;
use threadpool::ThreadPool;

use std::sync::Barrier;

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

    // create a barrier that wait for the thread to complete
    // in our case, we just let them run as long as allowed
    let barrier = Barrier::new(2);

    let pool = ThreadPool::new(workers);
    for n in 0..workers {
        pool.execute(move || {
            run_compute(n)
        });
    }
    barrier.wait();
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

    let batch_size: i32 = match env::var_os("BATCH_SIZE") {
        Some(val) => val.into_string().unwrap().parse().unwrap_or(0),
        None => panic!("BATCH_SIZE is not defined in the environment")
    };

    let mut count = 0;
    loop {
        do_compute();
        count=count+1;
        if count%batch_size == 0 {
            redis::cmd("INCRBY").arg(&key).arg(batch_size.to_string()).execute(&mut conn);
            info!("[{}] Computed {} operations", n, count);
        }
    }
}
