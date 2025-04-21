use yew::prelude::*;

mod table_component;
mod cell_component; // Use your local cell component
mod models; // Make sure this is included

use table_component::TableComponent;
// use cell_component::CellComponent; // Use your local cell component
// use models::{Cell, Sheet}; // Use your local models

#[function_component(App)]
fn app() -> Html {
    html! {
        <div style="font-family: sans-serif; padding: 2rem;">
            <h1 style="font-size: 2rem; margin-bottom: 1rem;">{ "🧮 Rust Spreadsheet" }</h1>
            <TableComponent />
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    gloo::utils::document().set_title("Rust Spreadsheet");
    console_error_panic_hook::set_once(); // Add this for better error reporting
    yew::Renderer::<App>::new().render();
}