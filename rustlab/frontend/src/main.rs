use yew::prelude::*;

mod cell_component;
mod context;
mod models;
mod request_form;
mod table_component;

use context::*;
use request_form::RequestForm;
use table_component::TableComponent;

#[function_component(App)]
fn app() -> Html {
    let app_state = use_reducer(AppState::default);

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

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    gloo::utils::document().set_title("Rust Spreadsheet");
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
