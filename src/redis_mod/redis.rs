use redis::aio::MultiplexedConnection;
use redis::AsyncCommands;
use serde_json::json;
use log::{info, error, debug};

pub async fn connect_redis(redis_url: &str) -> redis::RedisResult<MultiplexedConnection> {
    let client = redis::Client::open(redis_url)?;
    client.get_multiplexed_async_connection().await
}

pub fn normalize_chat_id(jid: &str) -> String {
    if let Some((number, domain)) = jid.split_once('@') {
        if number.starts_with("55") && number.len() >= 12 {
            let country_code = &number[..2];
            let area_code = &number[2..4];
            let rest = &number[4..];
            let rest = if rest.starts_with('9') {
                rest.to_string()
            } else {
                format!("9{}", rest)
            };
            let normalized_number = format!("{}{}{}", country_code, area_code, rest);
            return format!("{}@{}", normalized_number, domain);
        }
    }
    jid.to_string()
}

pub async fn ensure_chat_exists(
    redis_conn: &mut MultiplexedConnection,
    chat_id: &str,
    remote_jid: &str,
    chat_metadata: Option<&str>,
    message_data: Option<&[u8]>,
) -> redis::RedisResult<()> {
    let norm_chat_id = normalize_chat_id(chat_id);
    let chat_key = format!("chat:{}", norm_chat_id);
    let exists: bool = redis_conn.exists(&chat_key).await?;
    
    if !exists {
        let chat_data = if let Some(meta) = chat_metadata {
            meta.to_string()
        } else {
            let number = remote_jid.split('@').next().unwrap_or("").to_string();
            let instance_id = if let Some(data) = message_data {
                if let Ok(value) = serde_json::from_slice::<serde_json::Value>(data) {
                    value.get("apikey").and_then(|v| v.as_str()).unwrap_or("").to_string()
                } else {
                    "".to_string()
                }
            } else {
                "".to_string()
            };
            json!({
                "id": norm_chat_id,
                "situation": "enqueued",
                "is_active": true,
                "agent_id": null,
                "tabulation": null,
                "instance_id": instance_id,
                "number": number
            }).to_string()
        };
        let _: isize = redis_conn.rpush(&chat_key, chat_data).await?;
        info!("Created new chat entry in Redis (as list): {}", chat_key);
        let _: () = redis_conn.sadd("chats", &norm_chat_id).await?;
        info!("Added chat_id {} to 'chats' set", norm_chat_id);
    } else {
        debug!("Chat entry already exists in Redis: {}", chat_key);
    }
    Ok(())
}

pub async fn insert_message_to_chat(
    redis_conn: &mut MultiplexedConnection,
    chat_id: &str,
    message_json: &str,
    remote_jid: &str,
    chat_metadata: Option<&str>,
    message_data: Option<&[u8]>,
) -> redis::RedisResult<()> {
    let norm_chat_id = normalize_chat_id(chat_id);
    info!("Inserting message into chat:{} for remote_jid:{}", norm_chat_id, remote_jid);
    if let Err(e) = ensure_chat_exists(redis_conn, &norm_chat_id, remote_jid, chat_metadata, message_data).await {
        error!("Failed to ensure chat exists: {}", e);
        return Err(e);
    }
    let key = format!("chat:{}:messages", norm_chat_id);
    debug!("Pushing message to Redis list: {}", key);
    let _: isize = redis_conn.rpush(&key, message_json).await?;
    info!("Successfully inserted message into Redis for chat:{}", norm_chat_id);
    Ok(())
}

