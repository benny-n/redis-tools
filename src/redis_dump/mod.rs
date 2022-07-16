use std::collections::{HashMap, HashSet};

use anyhow::anyhow;
use redis::Commands;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::common::get_database_from_url;

#[derive(Default)]
pub enum DumpFilter {
    #[default]
    None,
    Keys(Vec<String>),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum RedisValue {
    String(String),
    Hash(HashMap<String, String>),
    List(Vec<String>),
    Set(HashSet<String>),
    ZSet(Vec<(String, f32)>),
}
pub struct RedisDump {
    conn: redis::Connection,
    db: u32,
    filter: DumpFilter,
}

impl RedisDump {
    pub fn new(url: Url) -> Result<Self, anyhow::Error> {
        let db = if let Some(db) = get_database_from_url(&url) {
            db
        } else {
            0
        };

        let conn = redis::Client::open(url)?.get_connection()?;
        Ok(Self {
            conn,
            db,
            filter: DumpFilter::None,
        })
    }
    pub fn with_db(mut self, db: u32) -> Self {
        self.db = db;
        self
    }
    pub fn with_filter(mut self, filter: DumpFilter) -> Self {
        self.filter = filter;
        self
    }
    pub fn conn(&self) -> &redis::Connection {
        &self.conn
    }
    pub fn conn_mut(&mut self) -> &mut redis::Connection {
        &mut self.conn
    }

    /// Select the active database.
    ///
    /// This is a no-op if the database is already selected.
    pub fn select_db(&mut self, db: u32) -> Result<(), anyhow::Error> {
        if self.db == db {
            return Ok(());
        }
        redis::cmd("SELECT").arg(db).query(&mut self.conn)?;
        self.db = db;
        Ok(())
    }

    /// Dump all keys in the active database.
    ///
    /// If a filter is set, only keys matching the filter will be dumped.
    ///
    /// Otherwise, all keys will be dumped.
    pub fn entries(&mut self) -> Result<HashMap<String, RedisValue>, anyhow::Error> {
        let mut entries = HashMap::<String, RedisValue>::new();
        let keys = self.conn.scan::<String>()?.collect::<Vec<_>>();

        for key in keys.iter() {
            let key_type: String = redis::cmd("TYPE").arg(key).query(&mut self.conn)?;
            // If the user specified which keys to dump, and this key is not in the list, skip it.
            if let DumpFilter::Keys(ref key_types) = self.filter {
                if !key_types.contains(&key_type) {
                    continue;
                }
            }

            let value = match key_type.as_str() {
                "string" => {
                    let value: String = self.conn.get(key)?;
                    RedisValue::String(value)
                }
                "list" => {
                    let value: Vec<String> = self.conn.lrange(key, 0, -1)?;
                    RedisValue::List(value)
                }
                "set" => {
                    let value: HashSet<String> = self.conn.smembers(key)?;
                    RedisValue::Set(value)
                }
                "hash" => {
                    let value: HashMap<String, String> = self.conn.hgetall(key)?;
                    RedisValue::Hash(value)
                }
                "zset" => {
                    let value: Vec<(String, f32)> = self.conn.zrange_withscores(key, 0, -1)?;
                    RedisValue::ZSet(value)
                }
                _ => {
                    return Err(anyhow!("{}: Unsupported type", key));
                }
            };
            entries.insert(key.to_string(), value);
        }
        Ok(entries)
    }
}
