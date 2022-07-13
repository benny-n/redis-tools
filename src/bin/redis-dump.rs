use anyhow::{anyhow, Result};
use clap::Parser;
use dotenv::dotenv;
use redis::{self, Commands};
use redis_tools::{cli::RedisDumpCli, common::ping, consts::*};
use serde::{Deserialize, Serialize};
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

pub fn dump_into_json(uri: String) -> Result<()> {
    let mut redis = HashMap::<String, RedisValue>::new();
    let mut conn = redis::Client::open(uri)?.get_connection()?;
    let keys = conn.scan::<String>()?.collect::<Vec<_>>();

    for key in keys {
        let output: String = redis::cmd("TYPE").arg(key.clone()).query(&mut conn)?;
        let value = match output.as_str() {
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
    println!("{}", serde_json::to_string_pretty(&redis)?);
    Ok(())
}

fn main() -> Result<(), anyhow::Error> {
    dotenv().ok();
    let args = RedisDumpCli::parse();

    let uri = if let Some(uri) = args.uri {
        uri
    } else if let Some(uri) = std::env::var(REDIS_URI_ENV_VAR_KEY).ok() {
        uri
    } else {
        REDIS_DEFAULT_URI.to_string()
    };

    let res = if args.ping {
        ping(uri)
    } else {
        dump_into_json(uri)
    };
    if let Err(err) = res {
        eprint!("{RED}");
        return Err(err);
    }
    return Ok(());
}
