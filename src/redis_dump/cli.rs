use clap::Parser;
use url::Url;

use redis_tools::{consts::{REDIS_DEFAULT_URL, REDIS_URL_ENV_VAR_KEY}, cli::{DbOption, is_number_or_all, key_type_exists}};


const REDIS_DUMP_EXAMPLES: &str = 
"\x1b[33mEXAMPLES\x1b[0m:
    \x1b[90m# Dump all databases into json\x1b[0m
    $ redis-dump \x1b[32m-d\x1b[0m all \x1b[91m>\x1b[0m dump.json

    \x1b[90m# Dump db 0 into json\x1b[0m
    $ redis-dump \x1b[32m-u\x1b[0m redis://localhost:6379/ \x1b[32m-d\x1b[0m 0 \x1b[91m>\x1b[0m dump.json
    
    \x1b[90m# Dump only string, list or hash keys of db 0 into json\x1b[0m
    $ redis-dump \x1b[32m-u\x1b[0m redis://localhost:6379/ \x1b[32m-d\x1b[0m 0 \x1b[32m--key-types\x1b[0m string list hash \x1b[91m>\x1b[0m dump.json

";

/// A tool for dumping Redis databases into a file
#[derive(Parser, Debug)]
#[clap(name = "redis-dump")]
#[clap(author, version, about, long_about = None)]
#[clap(after_help = REDIS_DUMP_EXAMPLES)]
pub struct RedisDumpCli {
    /// The redis server URL
    /// 
    /// The URL should usually be in the form of `redis://[:password]@host:port/db` or `redis://host:port/db`.
    /// The `db` part is optional, 0 being the default.
    /// Prefer specifying the database using the `-d` option.
    #[clap(short = 'u', long = "url", env = REDIS_URL_ENV_VAR_KEY, default_value = REDIS_DEFAULT_URL, value_parser = url::Url::parse, display_order = 0)]
    pub url: Url,
    /// The database to dump
    /// 
    /// Redis database name (usually 0-15), or `all` for all databases.
    /// If not specified, the database will be selected by the URL.
    /// If the URL does not specify a database, the default database (0) will be used.
    #[clap(name = "DB | all", short = 'd', long = "database", value_parser = is_number_or_all, display_order = 1)]
    pub db: Option<DbOption>,
    /// The key types to dump
    /// 
    /// Available key types: string, list, set, zset, hash.
    /// If `all` is specified, all key types will be dumped.
    #[clap(name = "TYPES", short = 'k', long = "key-types", value_parser = key_type_exists, min_values = 1, display_order = 2)]
    pub key_types: Option<Vec<String>>,
    /// Whether to include metadata (per-key) in the dump
    /// 
    /// If set, dump will NOT include metadata per key.
    /// The metadata consists of the key name, type, and ttl (time-to-live).
    #[clap(long = "no-metadata", value_parser, display_order = 3)]
    pub no_metadata: bool,
    /// Serialize the output as a pretty-printed JSON
    /// 
    /// NOTE: using this option will most likely increase the size of the output file.
    #[clap(short = 'p', long = "pretty", value_parser, display_order = 4)]
    pub pretty: bool,

    /// Prints this message.
    #[clap(short = 'h', long = "help", action = clap::ArgAction::Help,)]
    _help: Option<bool>,
    /// Prints the version.
    #[clap(short = 'v', long = "version", action = clap::ArgAction::Version)]
    _version: Option<bool>,
}


#[cfg(test)]
mod tests {
    use super::RedisDumpCli;
    use clap::IntoApp;

    #[test]
    fn redis_dump_cli_errors_test() {
        for op in ["--url", "--database", "--key-types"] {
            let res = RedisDumpCli::command().try_get_matches_from(&["redis-dump", op, "invalid"]);
            assert!(res.is_err());
            let err = res.unwrap_err();
            assert!(matches!(err.kind, clap::ErrorKind::ValueValidation));
        }
    }
}
