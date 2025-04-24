// server.rs
use log::{info, warn};
use std::io::Error;
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use tokio::net::TcpStream;

use futures_util::{SinkExt, StreamExt};
use tokio::sync::{
    RwLock,
    mpsc::{UnboundedSender, unbounded_channel},
};

use tokio_tungstenite::tungstenite::Message;

use crdt::{
    CLIENT_LIST, Client, ClientListEvent, Event, GRID_UPDATE, GridUpdateEvent, INIT, InitEvent,
};

type Clients = Arc<RwLock<HashMap<String, WsClient>>>;

#[derive(Debug, Clone)]
pub struct WsClient {
    pub name: String,
    pub sender: UnboundedSender<String>, // to facilitate sending messages to the client
}

async fn handle_init(
    evt: &InitEvent,
    clients: Clients,
    sender: UnboundedSender<String>,
    client_id: Arc<RwLock<Option<String>>>,
) {
    let name = evt.name.to_owned();
    *client_id.write().await = Some(name.clone());

    // add to clients list
    clients
        .as_ref()
        .write()
        .await
        .insert(name.clone(), WsClient { name: evt.name.to_owned(), sender: sender.clone() });

    // send updated list of clients to all clients
    let serialized = serde_json::to_string(&Event {
        event_type: CLIENT_LIST.to_string(),
        data: serde_json::to_value(ClientListEvent {
            clients: clients
                .read()
                .await
                .clone()
                .into_values()
                .map(|c| Client { name: c.name })
                .collect(),
        })
        .expect("Failed to serialize client list"),
    })
    .expect("Failed to serialize client list message");

    clients.read().await.iter().for_each(|client| {
        let _ = client.1.sender.send(serialized.clone());
    });

    info!("Client {} connected", name);
}

async fn handle_grid_update(evt: &GridUpdateEvent, clients: Clients) {
    let grid = evt.grid.clone();
    let client_message = Event {
        event_type: GRID_UPDATE.to_string(),
        data: serde_json::to_value(GridUpdateEvent {
            grid: grid.clone(),
            sender: evt.sender.clone(),
        })
        .expect("Failed to serialize grid update message"),
    };

    let serialized =
        serde_json::to_string(&client_message).expect("Failed to serialize grid update message");

    // send to each client in client list
    clients.read().await.iter().for_each(|client| {
        if client.0 != &evt.sender {
            let _ = client.1.sender.send(serialized.clone());
        }
    });

    info!("Grid update sent to all clients except {}", evt.sender);
}

async fn handle_close(clients: Clients, client_id: Arc<RwLock<Option<String>>>, addr: SocketAddr) {
    if let Some(ref name) = *client_id.read().await {
        clients.as_ref().write().await.remove(name); // remove client from list

        // send new list to all clients
        let serialized = serde_json::to_string(&Event {
            event_type: CLIENT_LIST.to_string(),
            data: serde_json::to_value(ClientListEvent {
                clients: clients
                    .read()
                    .await
                    .clone()
                    .into_values()
                    .map(|c| Client { name: c.name })
                    .collect(),
            })
            .expect("Failed to serialize client list"),
        })
        .expect("Failed to serialize client list message");

        clients.read().await.iter().for_each(|client| {
            let _ = client.1.sender.send(serialized.clone());
        });

        info!("Client {:?} disconnected at {}", name, addr);
    }
}

async fn accept_connection(stream: TcpStream, clients: Clients) {
    // create a new websocket connection
    let addr = stream.peer_addr().expect("Stream should have a peer address");

    info!("Client connected: {}", addr);

    // create a new websocket connection
    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake");

    info!("Websocket handshake successful: {}", addr);

    let (mut sender, mut receiver) = ws_stream.split();
    let (tx, mut rx) = unbounded_channel::<String>();
    let client_id: Arc<RwLock<Option<String>>> = Arc::new(RwLock::new(None));

    loop {
        tokio::select! {
            msg = receiver.next() => {
                match msg {
                    Some(msg) => {
                        let msg = msg.expect("Error parsing message");
                        if msg.is_text() {
                            if let Ok(event) = serde_json::from_str::<Event>(msg.to_text().expect("Error parsing message")) {
                                match event.event_type.as_str() {
                                    INIT => {
                                        if let Ok(evt) = serde_json::from_value::<InitEvent>(event.data) {
                                            handle_init(&evt, clients.clone(), tx.clone(), client_id.clone()).await;
                                        }
                                    },
                                    GRID_UPDATE => {
                                        if let Ok(evt) = serde_json::from_value::<GridUpdateEvent>(event.data) {
                                            handle_grid_update(&evt, clients.clone()).await;
                                        }
                                    },
                                    event_type => {
                                        warn!("Received unknown event: {}", event_type);
                                    }
                                }
                            }
                        } else if msg.is_close() {
                            handle_close(clients.clone(), client_id.clone(), addr).await;
                            break; // client has gracefully closed the connection
                        }
                    }
                    None => { // ungraceful close from client
                        info!("Client disconnected: {}", addr);
                        break;
                    }
                }
            },
            Some(ev) = rx.recv() => {
                sender.send(Message::Text(ev.to_owned())).await
                    .expect("Failed to send message to client");
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // initialize an empty hashmap to store clients
    info!("Initializing server...");
    let clients: Clients = Arc::new(RwLock::new(HashMap::new()));
    let _ = env_logger::try_init();

    info!("Starting server...");
    // bind to port 3030: this is server's listening port
    let listener = TcpListener::bind("127.0.0.1:3030").await.expect("Failed to bind");

    info!("Server is running on port 3030");

    // accept connections
    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream, clients.clone()));
    }

    Ok(())
}
