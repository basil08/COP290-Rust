use yew::prelude::*;
use sheet::Cell;
use crate::cell_component::CellComponent;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub row_index: usize,
    pub row_data: Vec<Cell>,
}

#[function_component(RowComponent)]
pub fn row_component(props: &Props) -> Html {
    html! {
        <tr>
            <td style="border: 1px solid #ccc; padding: 8px; background: #f9f9f9;">
                { props.row_index + 1 }
            </td>
            {
                props.row_data.iter().map(|cell| {
                    html! { <CellComponent cell={cell.clone()} /> }
                }).collect::<Html>()
            }
        </tr>
    }
}
