use yew::prelude::*;
use gloo::net::http::Request;
use wasm_bindgen_futures::spawn_local;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug, PartialEq)]
struct Cell {
    value: String,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
struct Sheet {
    data: Vec<Vec<Cell>>,
}

#[function_component(App)]
fn app() -> Html {
    let sheet_state = use_state(|| None::<Sheet>);

    {
        let sheet_state = sheet_state.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                if let Ok(response) = Request::get("http://127.0.0.1:3001/sheet")
                    .send()
                    .await
                {
                    if let Ok(sheet) = response.json::<Sheet>().await {
                        sheet_state.set(Some(sheet));
                    }
                }
            });
            || ()
        });
    }

    html! {
        <>
            <h1>{ "Simple Sheet Viewer" }</h1>
            {
                if let Some(sheet) = &*sheet_state {
                    html! {
                        <table border="1">
                            { for sheet.data.iter().map(|row| html! {
                                <tr>{ for row.iter().map(|cell| html! {
                                    <td>{ &cell.value }</td>
                                }) }</tr>
                            }) }
                        </table>
                    }
                } else {
                    html! { <p>{ "Loading..." }</p> }
                }
            }
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
