use clap::Parser;
use dotenv::dotenv;
use redis::{self, RedisError};
use redis_tools::{cli::RedisRestoreCli, common::ping};

#[allow(unused)]
pub fn restore_from_json(_uri: String) -> Result<(), RedisError> {
    unimplemented!()
}

fn main() {
    dotenv().ok();
    let uri = std::env::var("REDIS_URI").unwrap();
    let args = RedisRestoreCli::parse();

    if args.ping {
        ping(uri).unwrap();
    }
}