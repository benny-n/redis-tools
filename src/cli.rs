use clap::Parser;

/// A tool for dumping Redis databases into a file
#[derive(Parser, Debug)]
#[clap(name = "redis-dump")]
#[clap(author, version, about, long_about = None)]
pub struct RedisDumpCli {
    //// Ping the redis server
    #[clap(short = 'p', long = "ping", value_parser)]
    pub ping: bool,
    #[clap(short = 'u', long = "uri", value_parser)]
    pub uri: Option<String>,
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
