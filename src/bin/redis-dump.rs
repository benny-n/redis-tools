use anyhow::{anyhow, Result};
use clap::Parser;
use dotenv::dotenv;
use redis::{self, Commands};
use redis_tools::{cli::RedisDumpCli, common::get_uri, consts::RED};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;

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
    uri: String,
    keys_to_dump: Option<Vec<String>>,
) -> Result<JsonValue, anyhow::Error> {
    let mut redis = HashMap::<String, RedisValue>::new();
    let mut conn = redis::Client::open(uri)?.get_connection()?;
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
    Ok(serde_json::to_value(redis)?)
}

fn main() -> Result<(), anyhow::Error> {
    dotenv().ok();
    let args = RedisDumpCli::parse();

    let uri = get_uri(args.uri);

    let res = dump_into_json(uri, args.keys);
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
