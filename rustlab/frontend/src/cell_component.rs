use yew::prelude::*;
use sheet::Cell;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub cell: Cell,
}

#[function_component(CellComponent)]
pub fn cell_component(props: &Props) -> Html {
    let content = match &props.cell.value {
        sheet::CellValue::Int(i) => format!("{}", i),
        _ => "ERR".to_string(),
    };

    html! {
        <td style="border: 1px solid #ccc; padding: 8px;">
            { content }
        </td>
    }
}
