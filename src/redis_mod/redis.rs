use redis::aio::MultiplexedConnection;
use redis::AsyncCommands;

pub async fn connect_redis(redis_url: &str) -> redis::RedisResult<MultiplexedConnection> {
    let client = redis::Client::open(redis_url)?;
    client.get_multiplexed_async_connection().await
}

pub async fn insert_chat(redis: redis::RedisResult<MultiplexedConnection>) {
    
}

pub async fn insert_message_to_chat(redis_conn: &mut MultiplexedConnection, chat_id: &str, message_json: &str) -> redis::RedisResult<()> {
    let key = format!("chat:{}:messages", chat_id);
    let _: isize = redis_conn.rpush(key, message_json).await?;
    Ok(())
}