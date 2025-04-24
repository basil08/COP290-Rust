// lib.rs
//! A Conflict-free Replicated Data Type (CRDT) implementation for collaborative grid editing.
//! 
//! This library provides the core data structures and event types needed for building
//! a collaborative grid editing system. It implements CRDTs to ensure eventual consistency
//! across multiple clients.
//! 
//! # Examples
//! 
//! ```rust
//! use crdt::{Client, Event, InitEvent, GridUpdateEvent};
//! 
//! // Create a new client
//! let client = Client { name: "Alice".to_string() };
//! 
//! // Create an initialization event
//! let init_event = InitEvent { name: "Alice".to_string() };
//! 
//! // Create a grid update event
//! let grid_update = GridUpdateEvent {
//!     grid: vec![],
//!     sender: "Alice".to_string(),
//! };
//! ```
//! 
//! # Event Types
//! 
//! The library supports three main event types:
//! - `INIT`: For client initialization
//! - `GRID_UPDATE`: For grid content updates
//! - `CLIENT_LIST`: For broadcasting client list updates

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Event type for broadcasting client list updates to all connected clients.
pub const CLIENT_LIST: &str = "CLIENT_LIST";

/// Event type for client initialization.
pub const INIT: &str = "INIT";

/// Event type for grid content updates.
pub const GRID_UPDATE: &str = "GRID_UPDATE";

/// Represents a client in the collaborative system.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Client {
    /// The unique name/identifier of the client.
    pub name: String,
}

/// A generic event structure that can represent any type of event in the system.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    /// The type of event (e.g., INIT, GRID_UPDATE, CLIENT_LIST).
    pub event_type: String,
    /// The event-specific data payload.
    pub data: Value,
}

// two types of events:
// INIT event
// GRID_UPDATE event

/// Represents an initialization event sent when a new client joins the system.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InitEvent {
    /// The name of the client being initialized.
    pub name: String,
}

/// Represents a grid update event containing the latest state of the grid.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GridUpdateEvent {
    /// The updated grid content.
    pub grid: Vec<Row>,
    /// The name of the client who sent the update.
    pub sender: String,
}

/// Represents a client list update event broadcast to all clients.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientListEvent {
    /// The current list of all connected clients.
    pub clients: Vec<Client>,
}

/// Represents a single column in the grid with its metadata.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Column {
    /// The peer/client who last modified this column.
    pub peer: String,
    /// The timestamp of the last modification.
    pub timestamp: usize,
    /// The index of this column in the grid.
    pub idx: usize,
    /// The current value of the column.
    pub value: String,
}

/// Represents a row in the grid containing multiple columns.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Row {
    /// The index of this row in the grid.
    pub idx: usize,
    /// The columns contained in this row.
    pub columns: Vec<Column>,
}
