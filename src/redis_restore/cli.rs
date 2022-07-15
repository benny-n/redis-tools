use clap::Parser;

/// A tool for restoring Redis databases from a file
#[derive(Parser, Debug)]
#[clap(name = "redis-restore")]
#[clap(author, version, about, long_about = None)]
pub struct RedisRestoreCli {
    /// Ping the redis server
    #[clap(short = 'p', long = "ping", value_parser)]
    pub ping: bool,
}
