use redis::{ConnectionLike, RedisError};

pub fn ping(uri: String) -> Result<(), RedisError> {
    let mut conn = redis::Client::open(uri)?.get_connection()?;
    let res = conn.req_command(&redis::cmd("PING"))?;
    println!("{:#?}", res);
    Ok(())
}
