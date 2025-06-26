use tokio_postgres::{Client, Error};
use crate::parser::library::{Chat, Message};
use log::error;

pub async fn upsert_chats(client: Client, chat: Chat) -> Result<(), Error> {
    if let Some(tab) = chat.tabulation {
        match client.execute(
            "INSERT INTO chats (id, situation, is_active, agent_id, tabulation) VALUES ($1, $2, $3, $4, $5) ON CONFLICT (id) DO UPDATE SET situation = $2, is_active = $3, agent_id = $4, tabulation = $5",
            &[&chat.id, &chat.situation, &chat.is_active, &chat.agent_id, &tab]
        ).await {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("Error: Insertion on database failed: {}", e);
                Err(e)
            }
        }
    } else {
        match client.execute(
            "INSERT INTO chats (id, situation, is_active, agent_id) VALUES ($1, $2, $3, $4) ON CONFLICT (id) DO UPDATE SET situation = $2, is_active = $3, agent_id = $4",
            &[&chat.id, &chat.situation, &chat.is_active, &chat.agent_id]
        ).await {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("Error: Failed upsert on chats table: {}", e);
                Err(e)
            }
        }
    }
}   

pub async fn upsert_messages(client: Client, msg: Message) -> Result<(), Error> {
    match client.execute(
        "INSERT INTO messages (id, from, to, text, delivered) VALUES ($1, $2, $3, $4, $5) ON CONFLICT (id) DO UPDATE SET from = $2, to = $3, text = $4, delivered = $5",
        &[&msg.id, &msg.from, &msg.to, &msg.text, &msg.delivered]
    ).await {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("Error: Failed upsert on messages table: {}", e);
            Err(e)
        }
    }
}   