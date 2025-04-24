//! The binary entry point for the WebSocket client application.
//! 
//! This module provides the main function that bootstraps the Leptos application
//! and mounts the root `App` component to the web page's body.

#![allow(non_snake_case)]
use leptos::*;
use ws_client::App;

/// The main entry point for the WebSocket client application.
/// 
/// This function:
/// 1. Initializes the Leptos runtime
/// 2. Mounts the root `App` component to the document body
/// 3. Starts the reactive system for handling UI updates
fn main() {
    mount_to_body(|| view! { <App />})
}
