use std::io::Write;
use termcolor::{self, Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use url::Url;

pub fn get_database_from_url(url: &Url) -> Option<u32> {
    let db = url.path().split('/').collect::<Vec<&str>>();

    if db.is_empty() {
        return None;
    }

    db.get(1).cloned().and_then(|s| s.parse::<u32>().ok())
}

pub fn print_red_error() -> Result<StandardStream, anyhow::Error> {
    let stderr = StandardStream::stderr(ColorChoice::Auto);
    let mut stderr_lock = stderr.lock();
    stderr_lock.set_color(ColorSpec::new().set_bold(true).set_fg(Some(Color::Red)))?;
    write!(stderr_lock, "error: ")?;
    stderr_lock.set_color(&ColorSpec::new())?;
    drop(stderr_lock);
    Ok(stderr)
}

/// Returns the indices of DBs with at least 1 key.
pub fn get_all_non_empty_dbs(info_cmd_output: String) -> Vec<u32> {
    // Example of output for a redis-cli INFO keyspace command:
    // # Keyspace
    // db0:keys=4,expires=0,avg_ttl=0
    // db1:keys=1,expires=0,avg_ttl=0
    let mut db_indices = Vec::new();
    for line in info_cmd_output.lines() {
        if line.starts_with("db") {
            let db_index = line
                .split(':')
                .next()
                .map(|s| s.strip_prefix("db"))
                .flatten()
                .unwrap(); // SAFE UNWRAP: Due to if statement, line must start with "db"
            db_indices.push(
                db_index
                    .parse::<u32>() // For a valid redis-cli INFO keyspace output, db_index must be a valid u32.
                    .expect("corrupted redis-cli INFO keyspace command"),
            );
        }
    }
    db_indices
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
