use anyhow::{anyhow, Result};
use clap::{IntoApp, Parser};
use dotenv::dotenv;
use redis::{self, Commands};
use redis_tools::{
    cli::RedisDumpCli,
    common::{get_database_from_url, get_url},
    consts::{RED, REDIS_DEFAULT_URL},
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
    db: Option<u8>,
    keys_to_dump: Option<Vec<String>>,
) -> Result<JsonValue, anyhow::Error> {
    let mut redis_json = HashMap::<String, RedisValue>::new();
    let dbs = if let Some(db_from_arg) = db {
        // Prefer the db from the argument over the one from the url.
        vec![db_from_arg]
    } else if let Some(db_from_url) = get_database_from_url(&url) {
        // If a db was not explicitly specified in an argument, use the one from the url.
        vec![db_from_url]
    } else {
        // If a db was not specified at all, try to dump from all the default dbs (0..=15).
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
            "set" => {
                let value: Vec<String> = conn.smembers(key.clone())?;
                RedisValue::Set(value)
            }
            "hash" => {
                let value: HashMap<String, String> = conn.hgetall(key.clone())?;
                RedisValue::Hash(value)
            }
            "zset" => {
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
        // Print the error message and exit
        eprint!("{RED}");
        return Err(err);
    } else {
        maybe_url?
    };

    let res = dump_into_json(url, args.db, args.key_types);
    if let Err(err) = res {
        // Print the error message and exit
        eprint!("{RED}");
        Err(err)
    } else {
        // Print the JSON output of the Redis database to stdout (or to a file if output is redirected)
        println!("{}", serde_json::to_string_pretty(&res.ok())?);
        Ok(())
    }
}

#[test]
fn test_redis_info_keyspace_cmd() {
    let mut redis = HashMap::<String, RedisValue>::new();
    let mut conn = redis::Client::open(REDIS_DEFAULT_URL)
        .unwrap()
        .get_connection()
        .unwrap();
    let result: String = redis::cmd("INFO").arg("keyspace").query(&mut conn).unwrap();
    // Example of output:
    // # Keyspace
    // db0:keys=4,expires=0,avg_ttl=0
    // db1:keys=1,expires=0,avg_ttl=0

    // The next code parses the output and creates a vector of the db indices.

    let mut lines = result.lines();
    let mut db_indices: Vec<u32> = Vec::new();
    while let Some(line) = lines.next() {
        if line.starts_with("db") {
            let db_index = line.split(':').nth(0).unwrap().strip_prefix("db").unwrap();
            db_indices.push(db_index.parse::<u32>().unwrap());
        }
    }

    println!("{:?}", db_indices);
}
