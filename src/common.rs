use anyhow::Result;
use url::Url;

use crate::consts::{REDIS_DEFAULT_URL, REDIS_URL_ENV_VAR_KEY};

pub fn get_url(maybe_url: Option<Url>) -> Result<Url, anyhow::Error> {
    let url = if let Some(url) = maybe_url {
        Ok(url)
    } else if let Ok(url) = std::env::var(REDIS_URL_ENV_VAR_KEY) {
        Url::parse(url.as_str())
    } else {
        Url::parse(REDIS_DEFAULT_URL)
    }?;
    Ok(url)
}

pub fn get_database_from_url(url: &Url) -> Option<u32> {
    let db = url.path().split('/').collect::<Vec<&str>>();

    if db.is_empty() {
        return None;
    }

    db.get(1).cloned().and_then(|s| s.parse::<u32>().ok())
}

/// Returns the indices of DBs with at least 1 key.
pub fn get_non_empty_db_indices(url: &Url) -> Result<Vec<u32>> {
    let mut conn = redis::Client::open(url.to_string())?.get_connection()?;
    let result: String = redis::cmd("INFO").arg("keyspace").query(&mut conn)?;
    // Example of output for a redis-cli INFO keyspace command:
    // # Keyspace
    // db0:keys=4,expires=0,avg_ttl=0
    // db1:keys=1,expires=0,avg_ttl=0
    let mut db_indices = Vec::new();
    for line in result.lines() {
        if line.starts_with("db") {
            let db_index = line
                .split(':')
                .next()
                .unwrap() // Safe because we know the line cotains ':'
                .strip_prefix("db")
                .unwrap(); // Safe because we know the line starts with "db"
            db_indices.push(db_index.parse::<u32>()?);
        }
    }
    Ok(db_indices)
}

#[test]
fn get_database_test() {
    assert_eq!(
        get_database_from_url(&url::Url::parse("redis://localhost:6379").unwrap()),
        None
    );
    assert_eq!(
        get_database_from_url(&url::Url::parse("redis://localhost:6379/0").unwrap()),
        Some(0)
    );
    assert_eq!(
        get_database_from_url(&url::Url::parse("redis://localhost:6379/0/").unwrap()),
        Some(0)
    );
    assert_eq!(
        get_database_from_url(&url::Url::parse("redis://localhost:6379/0/foo").unwrap()),
        Some(0)
    );
    assert_eq!(
        get_database_from_url(&url::Url::parse("redis://localhost:6379/1/foo/").unwrap()),
        Some(1)
    );
    assert_eq!(
        get_database_from_url(&url::Url::parse("redis://localhost:6379/0/foo/bar").unwrap()),
        Some(0)
    );
    assert_eq!(
        get_database_from_url(&url::Url::parse("redis://localhost:6379/1/foo/bar/").unwrap()),
        Some(1)
    );
    assert_eq!(
        get_database_from_url(&url::Url::parse("redis://localhost:6379/asd/foo/bar/").unwrap()),
        None
    );
}
