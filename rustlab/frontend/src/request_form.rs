//! # Request Form Component
//! 
//! This module provides the form component that allows users to submit commands
//! to the backend API and displays the responses.

use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::context::{AppAction, AppContext};

/// Properties for the RequestForm component.
///
/// Contains the API URL endpoint that the form will submit requests to.
#[derive(Properties, PartialEq)]
pub struct RequestFormProps {
    /// The API endpoint URL for sending query requests
    pub api_url: String,
}

/// A form component for submitting commands to the backend.
///
/// This component provides:
/// - A text input for command entry
/// - A submit button to execute the command
/// - A response display area for showing API results
#[function_component(RequestForm)]
#[allow(unused_variables)]
pub fn request_form(props: &RequestFormProps) -> Html {
    let input_ref = use_node_ref();
    let response = use_state(String::new);
    let is_loading = use_state(|| false);

    // Get the app context for triggering refreshes
    let app_context = use_context::<AppContext>().expect("no ctx found");

    // Handler for form submission events.
    //
    // This callback:
    // 1. Prevents default form submission
    // 2. Gets the query text from the input
    // 3. Sends the query to the API asynchronously
    // 4. Updates the response state with the result
    // 5. Triggers app refresh on success
    let onsubmit = {
        let input_ref = input_ref.clone();
        let response = response.clone();
        let api_url = props.api_url.clone();
        let app_context = app_context.clone(); // Clone context for the closure

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                let query = input.value();
                if !query.trim().is_empty() {
                    let api_url = api_url.clone();
                    let response = response.clone();
                    let app_context = app_context.clone(); // Clone for the async closure

                    spawn_local(async move {
                        // Create the request with text/plain content type
                        let request = Request::post(&api_url).header("Content-Type", "text/plain");

                        // Handle the body() Result
                        let request_with_body = match request.body(query) {
                            Ok(req) => req,
                            Err(e) => {
                                response.set(format!("Failed to set request body: {:?}", e));
                                return;
                            }
                        };

                        // Send the request
                        match request_with_body.send().await {
                            Ok(resp) => {
                                // Parse the response as JSON
                                match resp.json::<serde_json::Value>().await {
                                    Ok(json) => {
                                        // Format JSON nicely
                                        match serde_json::to_string_pretty(&json) {
                                            Ok(formatted) => {
                                                response.set(formatted);

                                                // Trigger a refresh after successful operation
                                                app_context.dispatch(AppAction::Refresh);
                                            }
                                            Err(e) => response
                                                .set(format!("Error formatting JSON: {:?}", e)),
                                        }
                                    }
                                    Err(e) => {
                                        response.set(format!("Error parsing response: {:?}", e))
                                    }
                                }
                            }
                            Err(e) => response.set(format!("Request error: {:?}", e)),
                        }
                    });

                    input.set_value("");
                }
            }
        })
    };

    html! {
        <div style="margin-top: 20px; padding: 15px; border: 1px solid #ddd; border-radius: 5px; background-color: #f9f9f9;">
            <h3>{"Command Center"}</h3>
            <form {onsubmit} style="display: flex; gap: 10px;">
                <input
                    ref={input_ref}
                    type="text"
                    placeholder="Enter your command here..."
                    style="flex: 1; padding: 8px; border: 1px solid #ccc; border-radius: 4px;"
                />
                <button
                    type="submit"
                    style="padding: 8px 16px; background-color: #4CAF50; color: white; border: none; border-radius: 4px; cursor: pointer;"
                >
                    {"Execute"}
                </button>
            </form>

            <div style="margin-top: 10px;">
                {
                    if !(*response).is_empty() {
                        html! {
                            <div style="padding: 10px; border: 1px solid #ddd; border-radius: 4px; background-color: white;">
                                <h4>{"Response:"}</h4>
                                <pre style="white-space: pre-wrap; overflow-wrap: break-word; background-color: #f5f5f5; padding: 10px; border-radius: 4px;">
                                    {&*response}
                                </pre>
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }
            </div>
        </div>
    }
}