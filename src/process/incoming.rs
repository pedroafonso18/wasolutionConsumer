use crate::redis_mod::redis::{insert_message_to_chat, normalize_chat_id};
use redis::aio::MultiplexedConnection;
use serde_json::{Value, json};


pub async fn process_incoming(
    data: &[u8],
    redis_conn: &mut MultiplexedConnection,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let value: Value = serde_json::from_slice(data)?;

    let chat_id = value.pointer("/status_string/key/remote_jid")
        .or_else(|| value.pointer("/data/key/remoteJid"))
        .or_else(|| value.get("number"))
        .and_then(|v| v.as_str())
        .unwrap_or("unknown_chat");
    let chat_id = &normalize_chat_id(chat_id);
    let remote_jid = chat_id;

    let is_contact = value.get("name").is_some() && value.get("number").is_some() && value.get("created_at").is_some();
    let chat_metadata_string;
    let chat_metadata = if is_contact {
        let mut contact = value.clone();
        if contact.get("instance_id").is_none() {
            if let Some(instance_id) = value.get("instance_id").or_else(|| value.get("data").and_then(|d| d.get("instanceId"))) {
                contact["instance_id"] = instance_id.clone();
            }
        }
        chat_metadata_string = serde_json::to_string(&contact).unwrap_or("".to_string());
        Some(chat_metadata_string.as_str())
    } else {
        None
    };

    let (msg_id, from, to, text, body, msg_type, timestamp) = if let Some(data) = value.get("data") {
        let msg_id = data.pointer("/key/id").and_then(|v| v.as_str()).unwrap_or("");
        let from = value.get("sender").and_then(|v| v.as_str()).unwrap_or("");
        let to = data.pointer("/key/remoteJid").and_then(|v| v.as_str()).unwrap_or("");
        let mut text = data.pointer("/message/conversation").and_then(|v| v.as_str()).unwrap_or("");
        let mut body = text.to_string();
        let mut msg_type = data.get("messageType").and_then(|v| v.as_str()).unwrap_or("");
        let timestamp = value.get("date_time").and_then(|v| v.as_str()).unwrap_or("").to_string();

        if msg_type == "imageMessage" {
            msg_type = "image";
            let base64 = data.pointer("/message/base64").and_then(|v| v.as_str()).unwrap_or("");
            body = format!("data:image/png;base64,{}", base64);
            text = "📷 Imagem enviada";
        } else if msg_type == "audioMessage" {
            msg_type = "audio";
            let base64 = data.pointer("/message/base64").and_then(|v| v.as_str()).unwrap_or("");
            body = format!("data:audio/ogg;base64,{}", base64);
            text = "Áudio enviado";
        }
        (msg_id.to_string(), from.to_string(), to.to_string(), text.to_string(), body.to_string(), msg_type.to_string(), timestamp)
    } else {
        ("".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string())
    };
    let normalized = json!({
        "id": format!("msg_{}", msg_id),
        "from": from,
        "to": to,
        "text": text,
        "body": body,
        "type": msg_type,
        "timestamp": timestamp
    });
    let message_json = serde_json::to_string(&normalized).unwrap_or_default();
    insert_message_to_chat(redis_conn, chat_id, &message_json, remote_jid, chat_metadata, Some(data)).await?;


    Ok(())
}
