use std::collections::HashMap;

use anyhow::anyhow;
use redis::Commands;
use url::Url;

use crate::{
    __private::{consts::REDIS_DEFAULT_URL, utils::get_database_from_url},
    types::RedisValue,
};

#[derive(Default)]
pub enum RestoreFilter {
    #[default]
    None,
    Keys(Vec<String>),
}

pub struct RedisRestore {
    conn: redis::Connection,
    db: u32,
    //FIXME
    #[allow(dead_code)]
    filter: RestoreFilter,
}

pub struct RedisRestoreBuilder {
    url: Url,
    filter: RestoreFilter,
}

impl RedisRestoreBuilder {
    pub fn new() -> Self {
        Self {
            url: Url::parse(REDIS_DEFAULT_URL).expect("Default URL should be a valid URL"),
            filter: RestoreFilter::None,
        }
    }
    pub fn with_url(mut self, url: Url) -> Self {
        self.url = url;
        self
    }
    pub fn with_filter(mut self, filter: RestoreFilter) -> Self {
        self.filter = filter;
        self
    }
    pub fn connect(self) -> anyhow::Result<RedisRestore> {
        let conn = redis::Client::open(self.url.as_str())?.get_connection()?;
        let db = if let Some(db) = get_database_from_url(&self.url) {
            db
        } else {
            0
        };
        Ok(RedisRestore {
            conn,
            db,
            filter: self.filter,
        })
    }
}

impl RedisRestore {
    /// Build a new RedisRestore object.
    ///
    #[must_use]
    pub fn build() -> RedisRestoreBuilder {
        RedisRestoreBuilder::new()
    }

    /// Get the connection to the Redis server.
    ///
    pub fn conn(&self) -> &redis::Connection {
        &self.conn
    }

    /// Get (mutably) the connection to the Redis server.
    ///
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

    /// Restore the Redis database.
    pub fn fill_db(&mut self, entries: HashMap<String, RedisValue>) -> Result<(), anyhow::Error> {
        for (key, rv) in entries {
            if let RedisValue::Meta(metadata) = rv {
                self.select_db(metadata.db)?;
                match *metadata.data {
                    RedisValue::String(string) => {
                        self.conn.set(&key, string)?;
                    }
                    RedisValue::List(list) => {
                        self.conn.lpush(&key, list)?;
                    }
                    RedisValue::Set(set) => {
                        self.conn.sadd(&key, set)?;
                    }
                    RedisValue::Hash(hashmap) => {
                        let tupleslice =
                            hashmap.into_iter().map(|(k, v)| (k, v)).collect::<Vec<_>>();
                        self.conn.hset_multiple(&key, &tupleslice)?;
                    }
                    RedisValue::ZSet(zset) => {
                        let score_first_zset =
                            zset.into_iter().map(|(k, v)| (v, k)).collect::<Vec<_>>();
                        self.conn.zadd_multiple(&key, &score_first_zset)?;
                    }
                    _ => {
                        return Err(anyhow!("{}: Unsupported type", &key)); //TODO better error
                    }
                };
                if metadata.ttl > 0 {
                    self.conn.expire(key, metadata.ttl as usize)?;
                }
            };
        }
        Ok(())
    }
}
