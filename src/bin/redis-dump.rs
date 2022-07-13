use clap::Parser;
use dotenv::dotenv;
use redis::{self, RedisError};
use redis_tools::{cli::RedisDumpCli, common::ping};

#[allow(unused)]
pub fn dump_into_json(_uri: String) -> Result<(), RedisError> {
    unimplemented!()
}

fn main() {
    dotenv().ok();
    let uri = std::env::var("REDIS_URI").unwrap();
    let args = RedisDumpCli::parse();

    if args.ping {
        ping(uri).unwrap();
    }
}
