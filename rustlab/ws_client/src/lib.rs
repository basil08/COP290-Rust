use crate::wasm_bindgen::closure::Closure;
use crdt::{CLIENT_LIST, ClientListEvent, Column, Event, GRID_UPDATE, GridUpdateEvent, Row};
use leptos::wasm_bindgen::JsCast;
use leptos::{ev::SubmitEvent, html::Input, *};
use rand::prelude::*;
use web_sys::WebSocket;

// to communicate UI change to Leptos effect
// updates local state and sends event to other clients
#[derive(Debug, Clone)]
pub struct ChangeEvent {
    pub row: usize,
    pub column: usize,
    pub value: String,
}

#[component]
pub fn App() -> impl IntoView {
    let (ws, set_ws) = create_signal::<Option<WebSocket>>(None);
    let (clients, set_clients) = create_signal(vec![]);
    let (data_change, set_data_change) = create_signal::<Option<ChangeEvent>>(None);
    let (data, set_data) = create_signal(init_data());
    let (name, set_name) = create_signal(String::default());

    // Initialize WebSocket connection
    create_effect(move |_| {
        if ws.get().is_none() {
            let ws = WebSocket::new("ws://localhost:3030").expect("Failed to create WebSocket");
            // ws.set_binary_type(web_sys::BinaryType::Blob);

            // Set up message handler
            let set_clients_clone = set_clients;
            let set_data_clone = set_data;

            let onmessage = Closure::wrap(Box::new(move |ev: web_sys::MessageEvent| {
                if let Some(msg) = ev.data().as_string() {
                    if let Ok(evt) = serde_json::from_str::<Event>(&msg) {
                        if evt.event_type == CLIENT_LIST {
                            if let Ok(cl) = serde_json::from_value::<ClientListEvent>(evt.data) {
                                set_clients_clone.update(|c| {
                                    *c = cl
                                        .clients
                                        .into_iter()
                                        .map(|c| c.name)
                                        .collect::<Vec<String>>()
                                });
                            }
                        } else if evt.event_type == GRID_UPDATE {
                            if let Ok(m) = serde_json::from_value::<GridUpdateEvent>(evt.data) {
                                // simple last-write-wins CRDT merge logic
                                set_data_clone.update(|d| {
                                    for i in 0..d.len() {
                                        for j in 0..d[0].columns.len() {
                                            let local = &d[i].columns[j];
                                            let remote = &m.grid[i].columns[j];

                                            if local.timestamp > remote.timestamp {
                                                continue; // local version is newer - nothing to update
                                            }

                                            if local.timestamp == remote.timestamp && random() {
                                                continue; // timestamps are the same, use one at random
                                            }

                                            // overwrite local with remote
                                            d[i].columns[j] = m.grid[i].columns[j].clone();
                                        }
                                    }
                                });
                            }
                        }
                    }
                }
            }) as Box<dyn FnMut(_)>);

            ws.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
            onmessage.forget();

            set_ws.set(Some(ws));
        }
    });

    // effect for handling the sending of local updates to other clients via the websocket server
    create_effect(move |_| {
        if let Some(change) = data_change.get() {
            set_data_change.update(|dc| *dc = None);
            set_data.update(|d| {
                let old = &d[change.row].columns[change.column];
                let new = Column {
                    idx: old.idx,
                    peer: name.get(),
                    value: change.value,
                    timestamp: old.timestamp + 1,
                };
                d[change.row].columns[change.column] = new;
            });
            let d = data.get();

            let data_event = serde_json::to_value(GridUpdateEvent { grid: d, sender: name.get() })
                .expect("can serialize change event");
            let serialized = serde_json::to_string(&Event {
                event_type: GRID_UPDATE.to_owned(),
                data: data_event,
            })
            .expect("can be serialized");

            if let Some(ws) = ws.get() {
                if ws.ready_state() == WebSocket::OPEN {
                    ws.send_with_str(&serialized).expect("Failed to send message");
                }
            }
        }
    });

    view! {
        <div class="app">
            <div class="container">
                <span class="hidden">{move || data_change.get().is_some()}</span>
                <Connect ws={ws} set_name={set_name} />
                <Clients clients={clients} />
                <Grid data={data} set_data_change={set_data_change} />
            </div>
        </div>
    }
}

#[component]
pub fn Connect(ws: ReadSignal<Option<WebSocket>>, set_name: WriteSignal<String>) -> impl IntoView {
    let (connected, set_connected) = create_signal(false);
    let name_input: NodeRef<Input> = create_node_ref();

    let submit_handler = move |ev: SubmitEvent| {
        ev.prevent_default();
        let name = name_input.get().expect("input exists").value();
        if let Some(ws) = ws.get() {
            if ws.ready_state() == WebSocket::OPEN {
                ws.send_with_str(&format!(
                    r#"{{ "event_type": "INIT", "data": {{ "name": "{}" }} }}"#,
                    name
                ))
                .expect("Failed to send message");
                set_connected.update(|c| *c = true);
                set_name.update(|n| *n = name);
            }
        }
    };

    view! {
        <div class="connect">
            <div class="connect-name">
                <form on:submit=submit_handler>
                    <span>Name</span>
                    <span><input type="text" name="name" node_ref=name_input disabled=move || connected.get() /></span>
                    <span><input type="submit" disabled=move || connected.get() value="Connect" /></span>
                </form>
            </div>
        </div>
    }
}

#[component]
pub fn Clients(clients: ReadSignal<Vec<String>>) -> impl IntoView {
    view! {
        <div class="clients">
            <span>Clients</span>
            <ul class="clients-list">
                <For
                    each=move || clients.get()
                    key=|state| state.clone()
                    children=|child| view! { <li>{child}</li> }

                />
            </ul>
        </div>
    }
}

#[component]
pub fn Grid(
    data: ReadSignal<Vec<Row>>,
    set_data_change: WriteSignal<Option<ChangeEvent>>,
) -> impl IntoView {
    view! {
        <div class="grid-container">
            <table class="grid">
                <thead>
                    <tr>
                        <th></th> // Empty corner cell
                        {(0..10).map(|i| view! {
                            <th>{char::from_u32(65 + i as u32).unwrap()}</th>
                        }).collect_view()}
                    </tr>
                </thead>
                <tbody>
                    <For
                        each=move || data.get()
                        key=|r| r.idx
                        children=move |row| view! {
                            <tr>
                                <td class="row-header">{row.idx + 1}</td>
                                <For
                                    each=move || row.columns.clone()
                                    key=move |c| format!("{}{}", row.idx, c.idx)
                                    children=move |col| view! {
                                        <td>
                                            <input
                                                type="text"
                                                on:input=move |ev| {
                                                    set_data_change.update(|dc| *dc = Some(ChangeEvent {
                                                        row: row.idx,
                                                        column: col.idx,
                                                        value: event_target_value(&ev)
                                                    }));
                                                }
                                                prop:value=move || data.get()[row.idx].columns[col.idx].value.clone()
                                            />
                                        </td>
                                    }
                                />
                            </tr>
                        }
                    />
                </tbody>
            </table>
        </div>
    }
}

fn init_column(idx: usize) -> Column {
    Column { idx, value: String::default(), timestamp: 0, peer: String::default() }
}

pub fn init_data() -> Vec<Row> {
    (0..10).map(|i| Row { idx: i, columns: (0..10).map(init_column).collect() }).collect()
}
