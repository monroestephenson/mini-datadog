//! redis.rs
//! Module for Redis connection, caching, and rate limiting.

use redis::AsyncCommands;

pub async fn get_redis_connection(redis_url: &str) -> redis::RedisResult<redis::aio::Connection> {
    let client = redis::Client::open(redis_url)?;
    let conn = client.get_async_connection().await?;
    Ok(conn)
}

// Additional helper functions for caching, rate limiting