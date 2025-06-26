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