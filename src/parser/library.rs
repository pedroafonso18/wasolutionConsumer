use std::collections::HashMap;
use serde::Deserialize;


#[derive(Deserialize)]
pub struct Request {
    pub action: String,
    pub method: String,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub body: Option<HashMap<String, String>>,
    pub params: Option<HashMap<String, String>>
}

#[derive(Deserialize)]
pub struct Chat {
    pub id: i32,
    pub situation: String,
    pub is_active: bool,
    pub agent_id: Option<i32>,
    pub tabulation: Option<String>,
    pub customer_id: i32,
}

#[derive(Deserialize)]

pub struct Message {
    pub id: i32,
    pub from: String,
    pub to: String,
    pub delivered: bool,
    pub text: String,
    pub chat_id: i32
}

#[derive(Deserialize)]
pub struct Customer {
    pub id: i32,
    pub name: String,
    pub number: String,
    pub last_chat_id: Option<String>
}

#[derive(Deserialize)]
pub struct RabbitResponse {
    pub webhook: WebhookMessage,
    pub chat_id: String,
}

#[derive(Deserialize)]
pub struct WebhookMessage {
    pub headers: WebhookHeaders,
    pub params: HashMap<String, String>,
    pub query: HashMap<String, String>,
    pub body: WebhookBody,
    pub webhookUrl: String,
    pub executionMode: String,
}

#[derive(Deserialize)]
pub struct WebhookHeaders {
    pub host: String,
    #[serde(rename = "user-agent")]
    pub user_agent: String,
    #[serde(rename = "content-length")]
    pub content_length: String,
    #[serde(rename = "accept-encoding")]
    pub accept_encoding: String,
    #[serde(rename = "content-type")]
    pub content_type: String,
    #[serde(rename = "x-forwarded-for")]
    pub x_forwarded_for: String,
    #[serde(rename = "x-forwarded-host")]
    pub x_forwarded_host: String,
    #[serde(rename = "x-forwarded-port")]
    pub x_forwarded_port: String,
    #[serde(rename = "x-forwarded-proto")]
    pub x_forwarded_proto: String,
    #[serde(rename = "x-forwarded-server")]
    pub x_forwarded_server: String,
    #[serde(rename = "x-real-ip")]
    pub x_real_ip: String,
}

#[derive(Deserialize)]
pub struct WebhookBody {
    pub event: String,
    pub instance: String,
    pub data: WebhookData,
    pub destination: String,
    #[serde(rename = "date_time")]
    pub date_time: String,
    pub sender: String,
    #[serde(rename = "server_url")]
    pub server_url: String,
    pub apikey: String,
}

#[derive(Deserialize)]
pub struct WebhookData {
    pub key: MessageKey,
    #[serde(rename = "pushName")]
    pub push_name: String,
    pub message: MessageContent,
    #[serde(rename = "messageType")]
    pub message_type: String,
    #[serde(rename = "messageTimestamp")]
    pub message_timestamp: i64,
    #[serde(rename = "instanceId")]
    pub instance_id: String,
    pub source: String,
}

#[derive(Deserialize)]
pub struct MessageKey {
    #[serde(rename = "remoteJid")]
    pub remote_jid: String,
    #[serde(rename = "fromMe")]
    pub from_me: bool,
    pub id: String,
}

#[derive(Deserialize)]
pub struct MessageContent {
    pub conversation: Option<String>,
}
