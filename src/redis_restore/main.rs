mod cli;

use clap::Parser;
use cli::RedisRestoreCli;
use dotenv::dotenv;
use redis_tools::{redis_dump::RedisValue, utils::print_red_error};
use std::io::Write;

use std::{
    collections::HashMap,
    io::{self, Read},
};

pub fn cli_main(args: RedisRestoreCli) -> Result<(), anyhow::Error> {
    let mut buf = Vec::new();
    if let Some(file) = args.file {
        let mut file = std::fs::File::open(file)?;
        file.read_to_end(&mut buf)?;
    } else {
        io::stdin().read_to_end(&mut buf)?;
    }

    let redis_map: HashMap<String, RedisValue> = serde_json::from_slice(&buf)?;
    for (key, value) in redis_map {
        println!("{}: {:#?}", key, value);
    }
    Ok(())
}

fn main() -> Result<(), anyhow::Error> {
    // Load .env file if it exists.
    dotenv().ok();

    // Parse command line arguments, and run
    let args = RedisRestoreCli::parse();
    if let Err(err) = cli_main(args) {
        writeln!(print_red_error()?.lock(), "{}", err)?;
        std::process::exit(1);
    }
    Ok(())
}
