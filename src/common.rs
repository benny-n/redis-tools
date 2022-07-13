use anyhow::Result;
use redis::ConnectionLike;

pub fn ping(uri: String) -> Result<()> {
    let mut conn = redis::Client::open(uri)?.get_connection()?;
    let res = conn.req_command(&redis::cmd("PING"))?;
    println!("{:#?}", res);
    Ok(())
}
