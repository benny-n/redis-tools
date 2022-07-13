use anyhow::Result;
use redis::ConnectionLike;

use crate::consts::{REDIS_DEFAULT_URI, REDIS_URI_ENV_VAR_KEY};

pub fn get_uri(maybe_uri: Option<String>) -> String {
    if let Some(uri) = maybe_uri {
        uri
    } else if let Some(uri) = std::env::var(REDIS_URI_ENV_VAR_KEY).ok() {
        uri
    } else {
        REDIS_DEFAULT_URI.to_string()
    }
}

pub fn ping(uri: String) -> Result<()> {
    let mut conn = redis::Client::open(uri)?.get_connection()?;
    let res = conn.req_command(&redis::cmd("PING"))?;
    println!("{:#?}", res);
    Ok(())
}
