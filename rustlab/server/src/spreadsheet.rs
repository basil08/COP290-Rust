use std::sync::Arc;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::TryRecvError;
use std::sync::atomic::AtomicBool;
use tokio::sync::broadcast;
use tokio::sync::Notify;
use serde::{Deserialize, Serialize};
use warp::ws::Message;
use std::collections::HashMap;
use parking_lot::{RwLockUpgradableReadGuard, RwLock};

use operational_transform::OperationSeq;
use log::{info, warn};
use anyhow::{bail, Context, Error, Result};
use futures::prelude::*;

// use tokio::sync::mpsc;
use warp::ws::WebSocket;


pub struct Cell {
    pub value: String,
}

pub struct Spreadsheet {
    state: RwLock<State>,
    count: AtomicU64,
    notify: Notify,
    update: broadcast::Sender<ServerMsg>,
    killed: AtomicBool,
}

#[derive(Default)]
pub struct State {
    data: Vec<Vec<Cell>>,
    text: String,
    operations: Vec<UserOperation>,
    users: HashMap<u64, UserInfo>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserOperation {
    id: u64,
    operation: OperationSeq,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserInfo {
    name: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
enum ServerMsg {
    Identify(u64),
    History {
        start: usize,
        operations: Vec<UserOperation>,
    },
    UserInfo { id: u64, info: Option<UserInfo> },
}

impl From<ServerMsg> for Message {
    fn from(msg: ServerMsg) -> Self {
        let serialized = serde_json::to_string(&msg).expect("failed to serialize message");
        Message::text(serialized)
    }
}

impl Default for Spreadsheet {
    fn default() -> Self {
        let (tx, _) = broadcast::channel(16);
        Self {
            state: RwLock::new(State::default()),
            count: AtomicU64::new(0),
            notify: Notify::new(),
            update: tx,
            killed: AtomicBool::new(false),
        }
    }
}

#[derive(Serialize, Deserialize)]
enum ClientMsg {
    Edit {
        revision: usize,
        operation: OperationSeq,
    },
    ClientInfo(UserInfo),
}

impl Spreadsheet {
    
    // Helper functions
    /// Returns the current revision.
    pub fn revision(&self) -> usize {
        let state = self.state.read();
        state.operations.len()
    }

    /// Kill this object immediately, dropping all current connections.
    pub fn kill(&self) {
        self.killed.store(true, Ordering::Relaxed);
        self.notify.notify_waiters();
    }

    /// Returns if this Rustpad object has been killed.
    pub fn killed(&self) -> bool {
        self.killed.load(Ordering::Relaxed)
    }

    /// End of Helper functions
    

    pub async fn on_connection(&self, ws: WebSocket) {
        let id = self.count.fetch_add(1, Ordering::Relaxed);
        info!("{} connected", id);
        if let Err(e) = self.handle_connection(id, ws).await {
            warn!("{} disconnected: {}", id, e);
        }
        info!("{} disconnected", id);
        self.state.write().users.remove(&id);
        self.update
            .send(ServerMsg::UserInfo { id, info: None })
            .ok();
    }

    async fn handle_connection(&self, id: u64, mut ws: WebSocket) -> Result<()> {

        let mut update_rx = self.update.subscribe();

        let mut revision: usize = self.send_initial(id, &mut ws).await?;

        loop {
            let notified = self.notify.notified();
            if self.killed() {
                break;
            }
            if self.revision() > revision {
                revision = self.send_history(revision, &mut ws).await?;
            }

            tokio::select! {
                _ = notified => {}
                update = update_rx.recv() => {
                    ws.send(update?.into()).await?;
                }
                result = ws.next() => {
                    match result {
                        None => break,
                        Some(message) => {
                            self.handle_message(id, message?).await?;
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    

    async fn send_initial(&self, id: u64, ws: &mut WebSocket) -> Result<usize, Error> {
        ws.send(ServerMsg::Identify(id).into()).await?;
        let mut messages = Vec::new();
        let revision = {
            let state = self.state.read();
            if !state.operations.is_empty() {
                messages.push(ServerMsg::History {
                    start: 0,
                    operations: state.operations.clone(),
                });
            }
            for (&id, info) in &state.users {
                messages.push(ServerMsg::UserInfo {
                    id,
                    info: Some(info.clone()),
                });
            }
            state.operations.len()
        };
        for message in messages {
            ws.send(message.into()).await?;
        }
        Ok(revision)
    }

    async fn send_history(&self, start: usize, ws: &mut WebSocket) -> Result<usize, Error> {
        let operations = {
            let state = self.state.read();
            let len = state.operations.len();
            if start < len {
                state.operations[start..].to_owned()
            } else {
                Vec::new()
            }
        };
        let num_ops = operations.len();
        if num_ops > 0 {
            let msg = ServerMsg::History { start, operations};
            ws.send(msg.into()).await?;
        }
        Ok(start + num_ops)
    }

    async fn handle_message(&self, id: u64, message: Message) -> Result<(), Error> {
        let msg: ClientMsg = match message.to_str() {
            Ok(text) => serde_json::from_str(text).context("failed to deserialize message")?,
            Err(_) => return Ok(()), // Ignoring non-text messages
        };

        match msg {
            ClientMsg::Edit {
                revision,
                operation,
            } => {
                self.apply_edit(id, revision, operation)
                    .context("invalid edit operation")?;
                self.notify.notify_waiters();
            }
            ClientMsg::ClientInfo(info) => {
                self.state.write().users.insert(id, info.clone());
                let msg = ServerMsg::UserInfo {
                    id,
                    info: Some(info),
                };
                self.update.send(msg).ok();
            }
        }
        Ok(())
    }

    fn apply_edit(&self, id: u64, revision: usize, mut operation: OperationSeq) -> Result<(), Error> {
        info!(
            "edit: id = {}, revision = {}, base_len = {}, target_len = {}",
            id,
            revision,
            operation.base_len(),
            operation.target_len(),
        );

        let state = self.state.upgradable_read();
        let len = state.operations.len();
        if revision > len {
            bail!("got revision {}, but only {} revisions in history", revision, len);
        }
         
        for history_op in &state.operations[revision..] {
            operation = operation.transform(&history_op.operation)?.0;
        }

        if operation.target_len() > 256 * 1024 {
            bail!("edit is too large: {}", operation.target_len());
        }



        let new_text = operation.apply(&state.text)?;
        let mut state = RwLockUpgradableReadGuard::upgrade(state);

        state.operations.push(UserOperation { id, operation});
        state.text = new_text;
        Ok(())
    }
}


