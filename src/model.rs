// Serialize → Rust → JSON
// Deserialize → JSON → Rust
// Value → arbitrary JSON data 
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Endpoint {
    pub id: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: String,
    pub endpoint_id: String,
    pub payload: Value, // {"name": "Nikhil", "age": 21} or ["hello", "world"] or "hello"
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DeliveryStatus {
    Pending,
    Running,
    Succeeded,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Delivery {
    pub id: String,
    pub event_id: String,
    pub endpoint_id: String,
    pub status: DeliveryStatus,
    pub attempts: u32,
}

impl Endpoint {
    pub fn new(url: impl Into<String>) -> Self {
        let url = url.into();
        Self {
            id: format!("ep_{}", unique()), // here like ep_1234567890 smthg will be generated
            url,
        }
    }
}

impl Event {
    pub fn new(endpoint_id: impl Into<String>, payload: Value) -> Self {
        Self {
            id: format!("evt_{}", unique()),
            endpoint_id: endpoint_id.into(),
            payload,
        }
    }
}

impl Delivery {
    pub fn pending(event_id: impl Into<String>, endpoint_id: impl Into<String>) -> Self {
        Self {
            id: format!("dlv_{}", unique()),
            event_id: event_id.into(),
            endpoint_id: endpoint_id.into(),
            status: DeliveryStatus::Pending,
            attempts: 0,
        }
    }
}
 
// random unique id generator berlow | unique() is a cheap id.


fn unique() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let n = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("{n}")
}