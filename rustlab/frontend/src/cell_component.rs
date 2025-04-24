use crate::context::{AppAction, AppContext};
use gloo::console::log;
use gloo_net::http::Request;
use serde_json;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{FocusEvent, HtmlInputElement, InputEvent, KeyboardEvent};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub value: String,
    pub row_id: String,
    pub column_id: String,
    pub api_url: String,
}

#[function_component(CellComponent)]
pub fn cell_component(props: &Props) -> Html {
    let value = use_state(|| props.value.clone());
    let is_editing = use_state(|| false);
    let input_ref = use_node_ref(); // Add this line - use a node_ref instead

    // Get the app context for triggering refreshes
    let app_context = use_context::<AppContext>().expect("no ctx found");

    // Update the value if props change
    {
        let value = value.clone();
        let props_value = props.value.clone();

        use_effect_with(props.value.clone(), move |props_value| {
            value.set(props_value.clone());
            || ()
        });
    }

    // Focus and select input when editing is enabled
    {
        let input_ref = input_ref.clone();
        let is_editing = is_editing.clone();

        use_effect_with(*is_editing, move |is_editing| {
            if *is_editing {
                if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                    let _ = input.focus();
                    input.select();
                }
            }
            || ()
        });
    }

    let onclick = {
        let is_editing = is_editing.clone();

        Callback::from(move |_| {
            is_editing.set(true);
        })
    };

    let onblur = {
        let value = value.clone();
        let is_editing = is_editing.clone();
        let row_id = props.row_id.clone();
        let column_id = props.column_id.clone();
        let api_url = props.api_url.clone();
        let original_value = props.value.clone(); // Clone the original value here
        let app_context = app_context.clone(); // Clone for the closure

        Callback::from(move |_: FocusEvent| {
            let current_value = (*value).clone();
            let row_id = row_id.clone();
            let column_id = column_id.clone();
            let api_url = api_url.clone();
            let app_context = app_context.clone(); // Clone for the async closure

            // Only send update if the value has changed
            if current_value != original_value {
                spawn_local(async move {
                    let payload = serde_json::json!({
                        "row_id": row_id,
                        "column_id": column_id,
                        "value": current_value
                    });

                    // Create the request
                    let request =
                        Request::post(&api_url).header("Content-Type", "application/json");

                    // Handle the body() Result
                    let request_with_body =
                        match request.body(serde_json::to_string(&payload).unwrap()) {
                            Ok(req) => req,
                            Err(e) => {
                                log!("Failed to set request body: {:?}", e.to_string());
                                return;
                            }
                        };

                    // Send the request
                    match request_with_body.send().await {
                        Ok(_) => {
                            // Trigger a refresh after successful cell update
                            app_context.dispatch(AppAction::Refresh);
                        }
                        Err(e) => {
                            log!("Error updating cell: {:?}", e.to_string());
                            return;
                        }
                    }
                });
            }

            is_editing.set(false);
        })
    };

    let onkeypress = {
        let is_editing = is_editing.clone();

        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                e.prevent_default();
                // Simulate blur to trigger the update
                if let Some(target) = e.target() {
                    if let Some(input) = target.dyn_ref::<HtmlInputElement>() {
                        let _ = input.blur(); // Changed to handle the Result
                    }
                }
            }
        })
    };

    let oninput = {
        let value = value.clone();

        Callback::from(move |e: InputEvent| {
            if let Some(target) = e.target() {
                if let Some(input) = target.dyn_ref::<HtmlInputElement>() {
                    value.set(input.value());
                }
            }
        })
    };

    html! {
        <td style="border: 1px solid #ccc; padding: 8px; position: relative;">
            {
                if *is_editing {
                    html! {
                        <input
                            type="text"
                            value={(*value).clone()}
                            {oninput}
                            {onblur}
                            {onkeypress}
                            ref={input_ref}  // Use the node_ref here
                            style="width: 100%; height: 100%; padding: 5px; border: none; outline: none; position: absolute; top: 0; left: 0; box-sizing: border-box;"
                        />
                    }
                } else {
                    html! {
                        <div onclick={onclick} style="cursor: pointer; min-height: 20px;">
                            { (*value).clone() }
                        </div>
                    }
                }
            }
        </td>
    }
}
