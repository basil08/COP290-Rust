//! # Rust Spreadsheet Frontend
//!
//! This is the main module for the Rust Spreadsheet web application.
//! It provides a browser-based spreadsheet interface built with Yew
//! that communicates with a backend server for data processing.

use yew::prelude::*;

mod cell_component;
mod context;
mod models;
mod request_form;
mod table_component;

use context::*;
use request_form::RequestForm;
use table_component::TableComponent;

/// The root component of the Rust Spreadsheet application.
///
/// This component:
/// - Initializes the application state
/// - Sets up the main layout
/// - Renders the spreadsheet table and request form
#[function_component(App)]
fn app() -> Html {
    // Create application state using Yew's reducer pattern
    let app_state = use_reducer(AppState::default);

    // Backend API endpoint for queries
    let api_url = "http://localhost:3001/api/query".to_string();

    html! {
        <ContextProvider<AppContext> context={app_state.clone()}>
            <div style="font-family: sans-serif; padding: 2rem;">
                <h1 style="font-size: 2rem; margin-bottom: 1rem;">{ "🦀 Rust Spreadsheet" }</h1>
                <TableComponent />
                <RequestForm api_url={api_url} />
            </div>
        </ContextProvider<AppContext>>
    }
}

/// Application entry point.
///
/// Sets up:
/// - WebAssembly logging
/// - Document title
/// - Panic hook for better error reporting
/// - Renders the root component
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    gloo::utils::document().set_title("Rust Spreadsheet");
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}