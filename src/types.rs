use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RedisValue {
    String(String),
    Hash(HashMap<String, String>),
    List(Vec<String>),
    Set(HashSet<String>),
    ZSet(Vec<(String, f32)>),
    Meta(RedisMeta),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RedisMeta {
    pub(crate) db: u32,
    pub(crate) r#type: String,
    pub(crate) ttl: i64,
    pub(crate) data: Box<RedisValue>,
}
