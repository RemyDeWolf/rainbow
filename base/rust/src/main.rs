
mod compute;
extern crate redis;

use std::env;
use compute::do_compute;
use log::Level;

#[macro_use]
extern crate log;
extern crate simple_logger;

fn main() {
    simple_logger::init_with_level(Level::Info).unwrap();

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
            info!("Computed {} operations", count);
		}
    }
}
