mod cli;

use anyhow::Result;
use clap::{IntoApp, Parser};
use cli::RedisDumpCli;
use dotenv::dotenv;
use redis_tools::{
    cli::{self as cli_common},
    redis_dump::{DumpFilter, RedisDump, RedisValue},
    utils::get_all_non_empty_dbs,
};
use serde_json::Value as Json;
use std::collections::HashMap;

fn dump_json(mut rd: RedisDump) -> Result<Json, anyhow::Error> {
    Ok(serde_json::to_value(rd.entries()?)?)
}

fn dump_all_json(mut rd: RedisDump) -> Result<Json, anyhow::Error> {
    let mut redis_obj = HashMap::<String, RedisValue>::new();
    let dbs = get_all_non_empty_dbs(redis::cmd("INFO").arg("keyspace").query(rd.conn_mut())?);
    for db in dbs {
        rd.select_db(db)?;
        redis_obj.extend(rd.entries()?);
    }

    Ok(serde_json::to_value(redis_obj)?)
}

fn cli_main(args: RedisDumpCli) -> Result<Json, anyhow::Error> {
    let filter = if let Some(keys) = args.key_types {
        DumpFilter::Keys(keys)
    } else {
        DumpFilter::None
    };

    // Build the RedisDump object and connect to the server.
    let mut rd = RedisDump::build()
        .with_url(args.url)
        .with_filter(filter)
        .with_metadata(!args.no_metadata)
        .connect()?;

    // Select the database if it was specified.
    if let Some(cli_common::DbOption::Db(db)) = args.db {
        rd.select_db(db)?;
        dump_json(rd)
    // If `all` was specified, dump all databases.
    } else if let Some(cli_common::DbOption::All) = args.db {
        dump_all_json(rd)
    // If no database option was specified, the database will be selected by the URL.
    } else {
        dump_json(rd)
    }
}

fn main() -> Result<(), anyhow::Error> {
    // Load .env file if it exists.
    dotenv().ok();

    // Parse command line arguments, and run
    let args = RedisDumpCli::parse();
    let is_pretty = args.pretty;
    let json = cli_main(args);
    if let Err(err) = json {
        clap::Error::raw(clap::ErrorKind::Io, err.to_string())
            .format(&mut RedisDumpCli::into_app())
            .exit(); // We explicitly exit here to apply the error formatting.
    }

    // Print the JSON to stdout.
    if is_pretty {
        println!("{}", serde_json::to_string_pretty(&json?)?);
    } else {
        println!("{}", serde_json::to_string(&json?)?);
    }
    Ok(())
}
