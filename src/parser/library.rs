use std::collections::HashMap;

pub struct Request {
    pub action: String,
    pub method: String,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub body: Option<HashMap<String, String>>,
    pub params: Option<HashMap<String, String>>
}

pub struct Chat {
    pub id: i32,
    pub situation: String,
    pub is_active: bool,
    pub agent_id: Option<i32>,
    pub tabulation: Option<String>
}

pub struct Message {
    pub id: i32,
    pub from: String,
    pub to: String,
    pub delivered: bool,
    pub text: String,
}