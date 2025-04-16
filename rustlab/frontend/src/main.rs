
mod table_component;
mod row_component;
mod cell_component;


use table_component::TableComponent;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div style="font-family: sans-serif; padding: 2rem;">
            <h1 style="font-size: 2rem; margin-bottom: 1rem;">{ "🧮 Rust Spreadsheet" }</h1>
            <TableComponent rows={10} cols={10} />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
