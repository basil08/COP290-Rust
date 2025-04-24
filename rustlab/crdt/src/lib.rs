// lib.rs
use serde::{Deserialize, Serialize};
use serde_json::Value;

// constants
pub const CLIENT_LIST: &str = "CLIENT_LIST";
pub const INIT: &str = "INIT";
pub const GRID_UPDATE: &str = "GRID_UPDATE";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Client {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    pub event_type: String,
    pub data: Value,
}

// two types of events:
// INIT event
// GRID_UPDATE event

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InitEvent {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GridUpdateEvent {
    pub grid: Vec<Row>,
    pub sender: String,
}

// Broadcast to all clients
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientListEvent {
    pub clients: Vec<Client>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Column {
    pub peer: String,
    pub timestamp: usize,
    pub idx: usize,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Row {
    pub idx: usize,
    pub columns: Vec<Column>,
}
