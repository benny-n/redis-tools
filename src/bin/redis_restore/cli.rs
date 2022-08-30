use clap::Parser;
use redis_tools::__private::{
    cli_common::{is_number_or_all, key_type_exists, DbOption},
    consts::{REDIS_DEFAULT_URL, REDIS_URL_ENV_VAR_KEY},
};
use url::Url;

/// A tool for restoring Redis databases from a file
#[derive(Parser, Debug)]
#[clap(name = "redis-restore")]
#[clap(author, version, about, long_about = None)]
pub(crate) struct RedisRestoreCli {
    /// The redis server URL
    ///
    /// The URL should usually be in the form of `redis://[:password]@host:port/db` or `redis://host:port/db`.
    /// The `db` part is optional, 0 being the default.
    /// Prefer specifying the database using the `-d` option.
    #[clap(short = 'u', long = "url", env = REDIS_URL_ENV_VAR_KEY, default_value = REDIS_DEFAULT_URL, value_parser = url::Url::parse, display_order = 0)]
    pub(crate) url: Url,
    /// The database to restore from
    ///
    /// Redis database name (usually 0-15), or `all` for all databases.
    /// If not specified, the database will be selected by the URL.
    /// If the URL does not specify a database, the default database (0) will be used.
    #[clap(name = "DB | all", short = 'd', long = "database", value_parser = is_number_or_all, display_order = 1)]
    pub(crate) db: Option<DbOption>,
    /// The key types to restore
    ///
    /// Available key types: string, list, set, zset, hash.
    /// If `all` is specified, all key types will be restored.
    #[clap(name = "TYPES", short = 'k', long = "key-types", value_parser = key_type_exists, min_values = 1, display_order = 2)]
    pub(crate) key_types: Option<Vec<String>>,
    /// The file to restore from
    ///
    /// The file should be in JSON format.
    /// If not specified, the file will be read from stdin.
    #[clap(name = "PATH", short = 'f', long = "file", display_order = 3)]
    pub(crate) file: Option<String>,
}
