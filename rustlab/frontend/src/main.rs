use yew::prelude::*;
use sheet::{generate_grid, Cell, CellValue};

#[function_component(App)]
fn app() -> Html {
    let rows = 10;
    let cols = 10;
    let grid = generate_grid(rows, cols);

    let col_headers = (0..cols).map(|i| {
        let mut val = i + 1;
        let mut chars = vec![];
        while val > 0 {
            val -= 1;
            chars.push((b'A' + (val % 26) as u8) as char);
            val /= 26;
        }
        let header: String = chars.into_iter().rev().collect();
        html! { <th>{ header }</th> }
    });

    html! {
        <div>
            <h1>{ "Rust Spreadsheet Web UI" }</h1>
            <table border="1" style="border-collapse: collapse; text-align: center;">
                <thead>
                    <tr>
                        <th>{ "#" }</th>
                        { for col_headers }
                    </tr>
                </thead>
                <tbody>
                    { for (0..rows).map(|r| {
                        html! {
                            <tr>
                                <td><b>{ r + 1 }</b></td>
                                { for (0..cols).map(|c| {
                                    let cell = &grid[r][c];
                                    let content = if !cell.is_valid {
                                        "ERR".to_string()
                                    } else {
                                        match &cell.value {
                                            CellValue::Int(i) => i.to_string(),
                                            CellValue::Float(f) => format!("{:.2}", f),
                                            CellValue::String(s) => s.clone(),
                                        }
                                    };
                                    html! { <td>{ content }</td> }
                                })}
                            </tr>
                        }
                    })}
                </tbody>
            </table>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
