use gloo_net::http::Request;
use serde_json;
use wasm_bindgen_futures::spawn_local;
use web_sys::{InputEvent, KeyboardEvent};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct CellProps {
    pub value: String,
    pub row_id: String,
    pub column_id: String,
    pub api_url: String,
}

#[function_component(CellComponent)]
pub fn cell_component(props: &CellProps) -> Html {
    let is_editing = use_state(|| false);
    let value = use_state(|| props.value.clone());
    let original_value = use_state(|| props.value.clone());

    let on_double_click = {
        let is_editing = is_editing.clone();
        let value = value.clone();
        let original_value = original_value.clone();
        Callback::from(move |_| {
            original_value.set((*value).clone());
            is_editing.set(true);
        })
    };

    let on_input = {
        let value = value.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                value.set(input.value());
            }
        })
    };

    let send_update_to_server = {
        let value = value.clone();
        let original_value = original_value.clone();
        let row_id = props.row_id.clone();
        let column_id = props.column_id.clone();
        let api_url = props.api_url.clone();

        Callback::from(move |_| {
            if *value != *original_value {
                let value = (*value).clone();
                let row_id = row_id.clone();
                let column_id = column_id.clone();
                let api_url = api_url.clone();

                spawn_local(async move {
                    let payload = serde_json::json!({
                        "row_id": row_id,
                        "column_id": column_id,
                        "value": value
                    });

                    // Convert payload to string first
                    let json_string = match serde_json::to_string(&payload) {
                        Ok(s) => s,
                        Err(e) => {
                            log::error!("Failed to serialize JSON: {:?}", e);
                            return;
                        }
                    };

                    // Properly handle the Result from body()
                    let request =
                        Request::post(&api_url).header("Content-Type", "application/json");

                    // Handle the body() Result
                    let request_with_body = match request.body(json_string) {
                        Ok(req) => req,
                        Err(e) => {
                            log::error!("Failed to set request body: {:?}", e);
                            return;
                        }
                    };

                    // Now send the request
                    match request_with_body.send().await {
                        Ok(_) => log::info!("Cell updated successfully"),
                        Err(e) => log::error!("Failed to update cell: {:?}", e),
                    }
                });
            }
        })
    };

    let on_blur = {
        let is_editing = is_editing.clone();
        let update_server = send_update_to_server.clone();
        Callback::from(move |_| {
            update_server.emit(());
            is_editing.set(false);
        })
    };

    let on_key_press = {
        let is_editing = is_editing.clone();
        let update_server = send_update_to_server.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                update_server.emit(());
                is_editing.set(false);
            }
        })
    };

    html! {
        <td
            style="border: 1px solid #ccc; padding: 8px;"
            ondblclick={on_double_click}
        >
            {
                if *is_editing {
                    html! {
                        <input
                            type="text"
                            value={(*value).clone()}
                            oninput={on_input}
                            onblur={on_blur}
                            onkeypress={on_key_press}
                            autofocus=true
                        />
                    }
                } else {
                    html! { &*value }
                }
            }
        </td>
    }
}
