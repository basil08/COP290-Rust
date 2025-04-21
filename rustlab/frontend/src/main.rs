use yew::prelude::*;

mod table_component;
// mod row_component;
// mod cell_component;
mod models; // Make sure this is included

use table_component::TableComponent;
use models::{Cell, Sheet}; // Use your local models

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
    console_error_panic_hook::set_once(); // Add this for better error reporting
    yew::Renderer::<App>::new().render();
}