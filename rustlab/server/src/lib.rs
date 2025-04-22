mod spreadsheet;

use dashmap::DashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::{self, Instant};
use serde::{Deserialize, Serialize};
use log::info;

use crate::spreadsheet::Spreadsheet;
use warp::ws::WebSocket;
use warp::ws::Message;
use warp::{filters::BoxedFilter, ws::Ws, Filter, Rejection, Reply};

#[allow(dead_code)]
#[derive(Debug)]
struct CustomReject(anyhow::Error);

impl warp::reject::Reject for CustomReject {}


#[derive(Serialize, Deserialize)]
struct CellUpdate {
    row: usize,
    col: usize,
    value: String,
}

#[derive(Clone)]
pub struct Document {
    pub spreadsheet: Arc<Spreadsheet>,
    pub last_accessed: Instant,
}

impl Document {
    pub fn new(spreadsheet: Arc<Spreadsheet>) -> Self {
        Self {
            spreadsheet,
            last_accessed: Instant::now(),
        }
    }
}

impl Drop for Document {
    fn drop(&mut self) {
        self.spreadsheet.kill();
    }
}

/// Server State
#[derive(Clone)]
pub struct ServerState {
    pub documents: Arc<DashMap<String, Document>>,
}

/// Server configuration.
#[derive(Clone, Debug)]
pub struct ServerConfig {
    /// Number of days to clean up documents after inactivity.
    pub expiry_days: u32,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            expiry_days: 1,            
        }
    }
}

// =========================
// ===== Server Routes =====
// =========================

/// A combined filter handling all server routes.
pub fn server(config: ServerConfig) -> BoxedFilter<(impl Reply,)> {
    warp::path("api")
        .and(backend(config))
        .boxed()
}

/// Construct backend routes, including WebSocket handlers.
fn backend(config: ServerConfig) -> BoxedFilter<(impl Reply,)> {
    let state = ServerState {
        documents: Default::default(),
    };
    
    // a background task to clean up documents after a certain period of time
    tokio::spawn(cleaner(state.clone(), config.expiry_days));

    let state_filter = warp::any().map(move || state.clone());

    let socket = warp::path!("socket" / String)
        .and(warp::ws())
        .and(state_filter.clone())
        .and_then(socket_handler);

    socket.boxed()
}

/// Handler for the `/api/socket/{id}` endpoint.
async fn socket_handler(id: String, ws: Ws, state: ServerState) -> Result<impl Reply, Rejection> {
    use dashmap::mapref::entry::Entry;

    let mut entry = match state.documents.entry(id.clone()) {
        Entry::Occupied(e) => e.into_ref(),
        Entry::Vacant(e) => {
            let spreadsheet = Arc::new(Spreadsheet::default());
            e.insert(Document::new(spreadsheet))
        }
    };

    let value = entry.value_mut();
    value.last_accessed = Instant::now();
    let spreadsheet = Arc::clone(&value.spreadsheet);
    Ok(ws.on_upgrade(|socket| async move { spreadsheet.on_connection(socket).await }))
}


const HOUR: Duration = Duration::from_secs(3600);

/// Reclaims memory for documents.
async fn cleaner(state: ServerState, expiry_days: u32) {
    loop {
        time::sleep(HOUR).await;
        let mut keys = Vec::new();
        for entry in &*state.documents {
            if entry.last_accessed.elapsed() > HOUR * 24 * expiry_days {
                keys.push(entry.key().clone());
            }
        }
        info!("cleaner removing keys: {:?}", keys);
        for key in keys {
            state.documents.remove(&key);
        }
    }
}
