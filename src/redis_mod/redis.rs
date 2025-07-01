use redis::aio::MultiplexedConnection;
use redis::AsyncCommands;
use serde_json::json;

pub async fn connect_redis(redis_url: &str) -> redis::RedisResult<MultiplexedConnection> {
    let client = redis::Client::open(redis_url)?;
    client.get_multiplexed_async_connection().await
}

pub async fn ensure_chat_exists(redis_conn: &mut MultiplexedConnection, chat_id: &str, remote_jid: &str) -> redis::RedisResult<()> {
    let chat_key = format!("chat:{}", chat_id);
    let exists: bool = redis_conn.exists(&chat_key).await?;
    
    if !exists {
        let number = remote_jid.split('@').next().unwrap_or("").to_string();
        
        let chat_data = json!({
            "id": chat_id,
            "situation": "enqueued",
            "is_active": true,
            "agent_id": null,
            "tabulation": null,
            "instance_id": null,
            "number": number
        });
        
        let _: () = redis_conn.set(&chat_key, chat_data.to_string()).await?;
    }
    
    Ok(())
}

pub async fn insert_message_to_chat(redis_conn: &mut MultiplexedConnection, chat_id: &str, message_json: &str, remote_jid: &str) -> redis::RedisResult<()> {
    ensure_chat_exists(redis_conn, chat_id, remote_jid).await?;
    
    let key = format!("chat:{}:messages", chat_id);
    let _: isize = redis_conn.rpush(key, message_json).await?;
    Ok(())
}

