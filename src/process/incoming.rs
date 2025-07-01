use crate::parser::library::RabbitResponse;
use crate::redis_mod::redis::insert_message_to_chat;
use redis::aio::MultiplexedConnection;
use serde_json::json;

pub async fn process_incoming(data: &[u8], redis_conn: &mut MultiplexedConnection) -> Result<(), Box<dyn std::error::Error>> {
    let rabbit_response: RabbitResponse = serde_json::from_slice(data)?;

    let chat_id = &rabbit_response.chat_id;
    let sender = &rabbit_response.webhook.body.sender;
    let text = rabbit_response.webhook.body.data.message.conversation.clone().unwrap_or_default();
    let timestamp = &rabbit_response.webhook.body.date_time;
    let unix = rabbit_response.webhook.body.data.message_timestamp;

    let message = json!({
        "id": format!("msg_{}", unix),
        "from": sender,
        "to": chat_id,
        "text": text,
        "timestamp": timestamp
    });

    insert_message_to_chat(redis_conn, chat_id, &message.to_string()).await?;

    Ok(())
}
