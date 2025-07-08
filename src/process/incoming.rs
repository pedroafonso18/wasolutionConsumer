use crate::redis_mod::redis::insert_message_to_chat;
use redis::aio::MultiplexedConnection;
use serde_json::Value;

pub async fn process_incoming(data: &[u8], redis_conn: &mut MultiplexedConnection) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let value: Value = serde_json::from_slice(data)?;

    let chat_id = value.pointer("/status_string/key/remote_jid")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown_chat");
    let remote_jid = chat_id;
    let message_json = serde_json::to_string(&value).unwrap_or_default();
    insert_message_to_chat(redis_conn, chat_id, &message_json, remote_jid).await?;
    Ok(())
}
