use redis::aio::ConnectionManager;
use redis::{Client, RedisResult};
use std::env;

pub async fn init_redis() -> RedisResult<ConnectionManager> {
    let redis_url = env::var("REDIS_URL").unwrap();

    let client = Client::open(redis_url)?;
    let manager = ConnectionManager::new(client).await?;

    let pong: String = redis::cmd("PING").query_async(&mut manager.clone()).await?;

    if pong != "PONG" {
        return Err(redis::RedisError::from((
            redis::ErrorKind::IoError,
            "Redis PING failed",
        )));
    }

    println!("✅ Connecting to Redis is successful!");

    Ok(manager)
}
