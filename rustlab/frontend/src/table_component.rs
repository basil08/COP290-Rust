use yew::prelude::*;
use sheet::{generate_grid, Cell};
use crate::row_component::RowComponent;


#[derive(Properties, PartialEq)]
pub struct Props {
    pub rows: usize,
    pub cols: usize,
}

#[function_component(TableComponent)]
pub fn table_component(props: &Props) -> Html {
    let grid: Vec<Vec<Cell>> = generate_grid(props.rows, props.cols);

    html! {
        <table style="border-collapse: collapse; width: 100%; text-align: center;">
            <thead>
                <tr>
                    <th style="border: 1px solid #ccc; padding: 8px;">{ "↘" }</th>
                    { (1..=props.cols).map(|c| html! {
                        <th style="border: 1px solid #ccc; padding: 8px; background: #f0f0f0;">{ 
                            column_label(c - 1)
                         }</th>
                    }).collect::<Html>() }
                </tr>
            </thead>
            <tbody>
                {
                    (0..props.rows).map(|r| {
                        html! { <RowComponent row_index={r} row_data={grid[r].clone()} /> }
                    }).collect::<Html>()
                }
            </tbody>
        </table>
    }
}

pub fn column_label(mut index: usize) -> String {
    let mut label = String::new();
    index += 1; // Spreadsheet-style, 1-based

    while index > 0 {
        index -= 1;
        label.insert(0, (b'A' + (index % 26) as u8) as char);
        index /= 26;
    }

    label
}
