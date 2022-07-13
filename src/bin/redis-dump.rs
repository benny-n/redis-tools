use anyhow::{anyhow, Result};
use clap::Parser;
use dotenv::dotenv;
use redis::{self, Commands};
use redis_tools::{
    cli::RedisDumpCli,
    common::{get_database_from_url, get_url},
    consts::RED,
};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use url::Url;

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum RedisValue {
    String(String),
    Hash(HashMap<String, String>),
    List(Vec<String>),
    Set(Vec<String>),
    ZSet(Vec<(String, f32)>),
    Nil,
}

pub fn dump_into_json(
    url: Url,
    database: Option<u8>,
    keys_to_dump: Option<Vec<String>>,
) -> Result<JsonValue, anyhow::Error> {
    let mut redis_json = HashMap::<String, RedisValue>::new();
    let dbs = if let Some(db) = get_database_from_url(&url) {
        // If a db was specified in the url, use it.
        vec![db]
    } else if let Some(db) = database {
        // If a db was specified in the command line, use it.
        vec![db]
    } else {
        //Otherwise, use all the default dbs (0..=15).
        Vec::from_iter(0..=15)
    };

    for db in dbs {
        eprintln!("Dumping database: {}", db);
        let url = format!(
            "{}://{}:{}@{}:{}/{}",
            url.scheme(),
            url.username(),
            url.password().unwrap_or(""),
            url.host_str().ok_or(anyhow!("Missing host"))?,
            url.port().ok_or(anyhow!("Missing port"))?,
            db
        );
        redis_json.extend(dump_keys(url, &keys_to_dump)?);
    }

    Ok(serde_json::to_value(redis_json)?)
}

pub fn dump_keys(
    url: String,
    keys_to_dump: &Option<Vec<String>>,
) -> Result<HashMap<String, RedisValue>, anyhow::Error> {
    let mut redis = HashMap::<String, RedisValue>::new();
    let mut conn = redis::Client::open(url)?.get_connection()?;
    let keys = conn.scan::<String>()?.collect::<Vec<_>>();

    for key in keys {
        let key_type: String = redis::cmd("TYPE").arg(key.clone()).query(&mut conn)?;
        // If the user specified which keys to dump, and this key is not in the list, skip it.
        if let Some(ref key_types) = keys_to_dump {
            if !key_types.contains(&key_type) {
                continue;
            }
        }

        let value = match key_type.as_str() {
            "string" => {
                let value: String = conn.get(key.clone())?;
                RedisValue::String(value)
            }
            "list" => {
                let value: Vec<String> = conn.lrange(key.clone(), 0, -1)?;
                RedisValue::List(value)
            }
            "set" | "setex" => {
                let value: Vec<String> = conn.smembers(key.clone())?;
                RedisValue::Set(value)
            }
            "hash" | "hashex" => {
                let value: HashMap<String, String> = conn.hgetall(key.clone())?;
                RedisValue::Hash(value)
            }
            "zset" | "zsetex" => {
                let value: Vec<(String, f32)> = conn.zrange_withscores(key.clone(), 0, -1)?;
                RedisValue::ZSet(value)
            }
            _ => {
                return Err(anyhow!("{}: Unsupported type", key));
            }
        };
        redis.insert(key, value);
    }
    Ok(redis)
}

fn main() -> Result<(), anyhow::Error> {
    dotenv().ok();
    let args = RedisDumpCli::parse();

    let maybe_url = get_url(args.url);
    let url = if let Err(err) = maybe_url {
        // Print the error message and exit with error code 1
        eprint!("{RED}");
        return Err(err);
    } else {
        maybe_url?
    };

    let res = dump_into_json(url, args.db, args.keys);
    if let Err(err) = res {
        // Print the error message and exit with error code 1
        eprint!("{RED}");
        Err(err)
    } else {
        // Print the JSON output of the Redis database to stdout (or to a file if output is redirected)
        println!("{}", serde_json::to_string_pretty(&res.ok())?);
        Ok(())
    }
}
