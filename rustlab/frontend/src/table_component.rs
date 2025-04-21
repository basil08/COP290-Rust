use yew::prelude::*;
use gloo::net::http::Request;
use wasm_bindgen_futures::spawn_local;
use gloo::console::log;

use crate::models::{Cell, Sheet};

#[function_component(TableComponent)]
pub fn table_component() -> Html {
    let sheet_state = use_state(|| None::<Sheet>);
    let error_state = use_state(|| None::<String>);

    {
        let sheet_state = sheet_state.clone();
        let error_state = error_state.clone();
        use_effect_with((), move |_| {
            log!("Fetching sheet data...");
            spawn_local(async move {
                match Request::get("http://127.0.0.1:3001/sheet").send().await {
                    Ok(response) => {
                        log!("Got response with status:", response.status());
                        if response.status() == 200 {
                            match response.json::<Sheet>().await {
                                Ok(sheet) => {
                                    log!("Successfully parsed sheet data");
                                    sheet_state.set(Some(sheet));
                                    error_state.set(None);
                                },
                                Err(e) => {
                                    let err_msg = format!("Failed to parse response: {}", e);
                                    log!(err_msg.clone());
                                    error_state.set(Some(err_msg));
                                }
                            }
                        } else {
                            let err_msg = format!("Server returned error: {}", response.status());
                            log!(err_msg.clone());
                            error_state.set(Some(err_msg));
                        }
                    },
                    Err(e) => {
                        let err_msg = format!("Request failed: {}", e);
                        log!(err_msg.clone());
                        error_state.set(Some(err_msg));
                    }
                }
            });
            || ()
        });
    }

    html! {
        <div>
        {
            if let Some(error) = &*error_state {
                html! {
                    <div style="color: red; padding: 10px; border: 1px solid red; margin-bottom: 10px;">
                        <p><strong>{"Error: "}</strong>{error}</p>
                    </div>
                }
            } else {
                html! {}
            }
        }
        {
            if let Some(sheet) = &*sheet_state {
                html! {
                    <table style="border-collapse: collapse; width: 100%; text-align: center;">
                        <thead>
                            <tr>
                                <th style="border: 1px solid #ccc; padding: 8px;">{ "↘" }</th>
                                { (0..sheet.data[0].len()).map(|c| html! {
                                    <th style="border: 1px solid #ccc; padding: 8px; background:rgb(7, 188, 152);">
                                        { column_label(c) }
                                    </th>
                                }).collect::<Html>() }
                            </tr>
                        </thead>
                        <tbody>
                            {
                                sheet.data.iter().enumerate().map(|(r, row)| {
                                    html! {
                                        <tr>
                                            <td style="border: 1px solid #ccc; padding: 8px; background:rgb(7, 188, 152);">
                                                { r + 1 }
                                            </td>
                                            {
                                                row.iter().map(|cell| {
                                                    html! {
                                                        <td style="border: 1px solid #ccc; padding: 8px;">
                                                            { &cell.value }
                                                        </td>
                                                    }
                                                }).collect::<Html>()
                                            }
                                        </tr>
                                    }
                                }).collect::<Html>()
                            }
                        </tbody>
                    </table>
                }
            } else {
                html! { <p>{ "Loading sheet..." }</p> }
            }
        }
        </div>
    }
}

pub fn column_label(column: usize) -> String {
    let mut label = String::new();
    let mut index = column + 1;  // Convert to 1-based index
    
    while index > 0 {
        let remainder = (index - 1) % 26;
        label.insert(0, (b'A' + remainder as u8) as char);
        index = (index - 1) / 26;
    }
    
    label
}