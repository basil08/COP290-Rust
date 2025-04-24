use yew::prelude::*;
use gloo::net::http::Request;
use wasm_bindgen_futures::spawn_local;
use gloo::console::log;

// mod cell_component;
use sheet::function_ext::{Cell, CellValue};
use crate::models::*;
use crate::cell_component::CellComponent;

#[function_component(TableComponent)]
pub fn table_component() -> Html {
    let sheet_state = use_state(|| None::<Sheet>);
    let error_state = use_state(|| None::<String>);

    {
        let sheet_state = sheet_state.clone();
        let error_state = error_state.clone();
        use_effect_with((), move |_| {
            log!("Fetching sheet data...");
            spawn_local(async move {
                match Request::get("http://127.0.0.1:3001/sheet").send().await {
                    Ok(response) => {
                        if response.status() == 200 {
                            match response.json::<Sheet>().await {
                                Ok(sheet) => {
                                    sheet_state.set(Some(sheet));
                                    error_state.set(None);
                                },
                                Err(e) => {
                                    error_state.set(Some(format!("Parse error: {}", e)));
                                }
                            }
                        } else {
                            error_state.set(Some(format!("Server error: {}", response.status())));
                        }
                    },
                    Err(e) => {
                        error_state.set(Some(format!("Request failed: {}", e)));
                    }
                }
            });
            || ()
        });
    }

    html! {
        <div>
        {
            if let Some(error) = &*error_state {
                html! { <p style="color: red;">{ error }</p> }
            } else if let Some(sheet) = &*sheet_state {
                html! {
                    <table style="border-collapse: collapse; width: 100%; text-align: center;">
                        <thead>
                            <tr>
                                <th style="border: 1px solid #ccc; padding: 8px;">{ "↘" }</th>
                                {
                                    (0..sheet.data[0].len()).map(|c| html! {
                                        <th style="border: 1px solid #ccc; padding: 8px; background:rgb(7, 188, 152);">
                                            { column_label(c) }
                                        </th>
                                    }).collect::<Html>()
                                }
                            </tr>
                        </thead>
                        <tbody>
                            {
                                sheet.data.iter().enumerate().map(|(r, row)| {
                                    html! {
                                        <tr>
                                            <td style="border: 1px solid #ccc; padding: 8px; background:rgb(7, 188, 152);">{ r + 1 }</td>
                                            {   
                                                row.iter().enumerate().map(|(c, cell)| {
                                                    let display_value = match &cell.value {
                                                        CellValue::Int(i) => i.to_string(),
                                                        CellValue::Float(f) => f.to_string(),
                                                        CellValue::String(s) => s.clone(),
                                                    };

                                                    html! {
                                                        <CellComponent
                                                                value={display_value}
                                                                row_id={r.to_string()}
                                                                column_id={c.to_string()}
                                                                api_url={"http://127.0.0.1:3001/update-cell".to_string()}
                                                        />  
                                                    }
                                                }).collect::<Html>()
                                            }
                                        </tr>
                                    }
                                }).collect::<Html>()
                            }
                        </tbody>
                    </table>
                }
            } else {
                html! { <p>{ "Loading..." }</p> }
            }
        }
        </div>
    }
}

pub fn column_label(mut index: usize) -> String {
    let mut label = String::new();
    index += 1;
    while index > 0 {
        index -= 1;
        label.insert(0, (b'A' + (index % 26) as u8) as char);
        index /= 26;
    }
    label
}
