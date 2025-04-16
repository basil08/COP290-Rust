use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <h1>{ "Rust Spreadsheet Web UI" }</h1>
            <h1 style="color: black; font-size: 35px;">{ "hello world computer" }</h1>
            <p>{ "Welcome to the browser version!" }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

