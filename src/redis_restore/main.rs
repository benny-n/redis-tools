mod cli;

use clap::Parser;
use cli::RedisRestoreCli;
use dotenv::dotenv;
use redis::{self, RedisError};

#[allow(unused)]
pub fn restore_from_json(_uri: String) -> Result<(), RedisError> {
    unimplemented!()
}

fn main() {
    dotenv().ok();
    let uri = std::env::var("REDIS_URI").unwrap();
    let args = RedisRestoreCli::parse();
    unimplemented!("{}, {:?}", uri, args);
}
