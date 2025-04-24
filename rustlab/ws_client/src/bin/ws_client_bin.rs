#![allow(non_snake_case)]
use leptos::*;
use ws_client::App;

fn main() {
    mount_to_body(|| view! { <App />})
}
