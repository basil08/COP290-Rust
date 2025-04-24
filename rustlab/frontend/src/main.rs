use yew::prelude::*;

mod table_component;
mod cell_component;
mod models;
mod request_form;

use table_component::TableComponent;
use request_form::RequestForm;

#[function_component(App)]
fn app() -> Html {
    // You might want to define your API URL here or get it from a configuration
    let api_url = "http://localhost:3001/api/query".to_string();
    
    html! {
        <div style="font-family: sans-serif; padding: 2rem;">
            <h1 style="font-size: 2rem; margin-bottom: 1rem;">{ "🦀 Rust Spreadsheet" }</h1>
            <TableComponent />
            <RequestForm api_url={api_url} />
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    gloo::utils::document().set_title("Rust Spreadsheet");
    console_error_panic_hook::set_once(); 
    yew::Renderer::<App>::new().render();
}