use std::ops::RangeInclusive;

use clap::Parser;
use url::Url;

use crate::consts::REDIS_KEY_TYPE;

const REDIS_DUMP_EXAMPLES: &str = "
\x1b[33mEXAMPLES\x1b[0m:
    $ redis-dump \x1b[91m>\x1b[0m dump.json
    $ redis-dump \x1b[32m-u\x1b[0m redis://localhost:6379/0 \x1b[91m>\x1b[0m dump.json
    $ redis-dump \x1b[32m-u\x1b[0m redis://localhost:6379/ \x1b[32m-d\x1b[0m 0 \x1b[32m--key-types\x1b[0m string list hash \x1b[91m>\x1b[0m dump.json

";

/// A tool for dumping Redis databases into a file
#[derive(Parser, Debug)]
#[clap(name = "redis-dump")]
#[clap(author, version, about, long_about = None)]
#[clap(after_help = REDIS_DUMP_EXAMPLES)]
pub struct RedisDumpCli {
    /// The redis server URL
    #[clap(short = 'u', long = "url", value_parser = url::Url::parse)]
    pub url: Option<Url>,
    /// The database to dump
    #[clap(short = 'd', long = "database", value_parser = db_idx_in_range)]
    pub db: Option<u8>,
    /// The key types to dump (if not specified, all keys will be dumped)
    #[clap(name = "TYPES", short = 'k', long = "key-types", value_parser = key_type_exists, min_values = 1)]
    pub key_types: Option<Vec<String>>,
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

fn key_type_exists(s: &str) -> Result<String, String> {
    if REDIS_KEY_TYPE.contains(&s) {
        Ok(s.to_string())
    } else {
        Err(format!(
            "Redis key type `{}` is not one of: {}",
            s,
            REDIS_KEY_TYPE.join(", ")
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::cli::RedisDumpCli;
    use clap::IntoApp;

    #[test]
    fn redis_dump_cli_test() {
        let res = RedisDumpCli::command().try_get_matches_from(&[
            "redis-dump",
            "--url",
            "redis://localhost:6379",
            "--database",
            "0",
            "--keys",
            "foo",
            "bar",
        ]);
        println!("{:#?}", res);
        // let cli = RedisDumpCli::parse_from(&["redis-dump", "--url", "a"]);
        // assert_eq!(cli.url.unwrap().as_str(), "redis://localhost:6379");
        // assert_eq!(cli.db, None);
        // assert_eq!(cli.keys, None);
        // let arg_vec = vec!["my_prog", "some", "args", "to", "parse"];

        // let _matches = clap::App::new("myprog")
        //     // Args and options go here...
        //     .try_get_matches_from(arg_vec)
        //     .unwrap_or_else(|e| e.exit());
    }
}
