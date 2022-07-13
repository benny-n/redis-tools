use std::ops::RangeInclusive;

use clap::Parser;
use url::Url;

/// A tool for dumping Redis databases into a file
#[derive(Parser, Debug)]
#[clap(name = "redis-dump")]
#[clap(author, version, about, long_about = None)]
pub struct RedisDumpCli {
    /// The redis server URL
    #[clap(short = 'u', long = "url", value_parser = url::Url::parse)]
    pub url: Option<Url>,
    /// The database to dump
    #[clap(short = 'd', long = "database", value_parser = db_idx_in_range)]
    pub db: Option<u8>,
    /// The keys to dump (if not specified, all keys will be dumped)
    #[clap(short = 'k', long = "keys", value_parser, min_values = 1)]
    pub keys: Option<Vec<String>>,
}

/// A tool for restoring Redis databases from a file
#[derive(Parser, Debug)]
#[clap(name = "redis-restore")]
#[clap(author, version, about, long_about = None)]
pub struct RedisRestoreCli {
    /// Ping the redis server
    #[clap(short = 'p', long = "ping", value_parser)]
    pub ping: bool,
}

const DB_IDX_RANGE: RangeInclusive<usize> = 0..=15;

fn db_idx_in_range(s: &str) -> Result<u8, String> {
    let db_idx: usize = s
        .parse()
        .map_err(|_| format!("`{}` isn't a valid Redis DB index", s))?;
    if DB_IDX_RANGE.contains(&db_idx) {
        Ok(db_idx as u8)
    } else {
        Err(format!(
            "Redis DB index out of range {}-{}",
            DB_IDX_RANGE.start(),
            DB_IDX_RANGE.end()
        ))
    }
}
