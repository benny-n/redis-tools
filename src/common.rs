use url::Url;

use crate::consts::{REDIS_DEFAULT_URL, REDIS_URL_ENV_VAR_KEY};

pub fn get_url(maybe_url: Option<Url>) -> Result<Url, anyhow::Error> {
    let url = if let Some(url) = maybe_url {
        Ok(url)
    } else if let Some(url) = std::env::var(REDIS_URL_ENV_VAR_KEY).ok() {
        Url::parse(url.as_str())
    } else {
        Url::parse(REDIS_DEFAULT_URL)
    }?;
    Ok(url)
}

pub fn get_database_from_url(url: &Url) -> Option<u8> {
    let db = url.path().split("/").collect::<Vec<&str>>();

    if db.is_empty() {
        return None;
    }

    db.get(1).cloned().map(|s| s.parse::<u8>().ok()).flatten()
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
