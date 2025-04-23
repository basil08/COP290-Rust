use sheet::graph_ext::{Graph, Formula, State};
use sheet::function_ext::{Cell, CellValue};
use sheet::display_ext::{printer, scroller};

#[test]
fn test_add_and_delete_edge() {
    let mut graph = Graph::new(5);
    graph.add_edge(2, 0);
    graph.add_edge(3, 0);
    let mut formulas = vec![Formula::default(); 5];
    formulas[2] = Formula { op_type: 1, op_info1: 0, op_info2: 0 };
    graph.delete_edge(2, 5, &formulas);
    assert_eq!(graph.adj_lists_head[0].as_ref().unwrap().cell, 3);
}

// #[test]
// fn test_add_and_delete_range() {
//     let mut graph = Graph::new(5);
//     graph.add_range_to_graph(0, 1, 2);
//     graph.add_range_to_graph(3, 4, 2);
//     graph.delete_range_from_graph(2);
//     assert!(graph.ranges_head.is_none());
// }

#[test]
fn test_printer_prints_valid_int_cells() {
    let currx = 0;
    let curry = 0;
    let c = 5;
    let r = 5;
    let mut arr = vec![Cell::new_int(0); (c * r) as usize];
    arr[2] = Cell::new_int(42);
    printer(currx, curry, &arr, c, r);
}

#[test]
fn test_printer_prints_float_and_string_cells() {
    let currx = 0;
    let curry = 0;
    let c = 5;
    let r = 5;
    let mut arr = vec![Cell::new_int(0); (c * r) as usize];
    arr[1] = Cell::new_float(3.14159);
    arr[2] = Cell::new_string("test".to_string());
    arr[3] = Cell::new_string("longer-than-ten-chars".to_string());
    printer(currx, curry, &arr, c, r);
}

#[test]
fn test_printer_with_invalid_cells() {
    let currx = 0;
    let curry = 0;
    let c = 2;
    let r = 2;
    let mut arr = vec![Cell::new_int(0); 4];
    arr[0] = Cell::invalid();
    printer(currx, curry, &arr, c, r);
}

#[test]
fn test_scroller_with_unknown_command() {
    let mut currx = 0;
    let mut curry = 0;
    let c = 10;
    let r = 10;
    let arr = vec![Cell::new_int(0); (c * r) as usize];
    let graph = Graph::new((c * r) as usize);
    let result = scroller("unknown", &arr, &mut currx, &mut curry, c, r, &graph);
    assert!(result.is_err());
}

#[test]
fn test_scroller_scroll_to_out_of_bounds_cell() {
    let mut currx = 0;
    let mut curry = 0;
    let c = 5;
    let r = 5;
    let arr = vec![Cell::new_int(0); (c * r) as usize];
    let graph = Graph::new((c * r) as usize);
    let result = scroller("scroll_to ZZ99", &arr, &mut currx, &mut curry, c, r, &graph);
    assert!(result.is_err());
}

#[test]
fn test_scroller_scroll_to_cell_row_col_boundary() {
    let mut currx = 0;
    let mut curry = 0;
    let c = 26;
    let r = 26;
    let arr = vec![Cell::new_int(0); (c * r) as usize];
    let graph = Graph::new((c * r) as usize);
    let result = scroller("scroll_to A1", &arr, &mut currx, &mut curry, c, r, &graph);
    assert!(result.is_ok());
    assert_eq!((currx, curry), (0, 0));
}

#[test]
fn test_printer_empty_grid() {
    let currx = 0;
    let curry = 0;
    let c = 3;
    let r = 3;
    let arr = vec![Cell::default(); (c * r) as usize];
    printer(currx, curry, &arr, c, r);
}

#[test]
fn test_scroller_scroll_to_valid_cell() {
    let mut currx = 0;
    let mut curry = 0;
    let c = 26;
    let r = 30;
    let arr = vec![Cell::new_int(0); (c * r) as usize];
    let graph = Graph::new((c * r) as usize);
    let result = scroller("scroll_to Z30", &arr, &mut currx, &mut curry, c, r, &graph);
    assert!(result.is_ok());
    assert_eq!((currx, curry), (25, 29));
}

#[test]
fn test_scroller_scroll_and_print() {
    let mut currx = 0;
    let mut curry = 0;
    let c = 10;
    let r = 10;
    let mut arr = vec![Cell::default(); (c * r) as usize];
    arr[11] = Cell::new_string("xyz".into()); // B2
    let graph = Graph::new((c * r) as usize);
    let _ = scroller("scroll_to B2", &arr, &mut currx, &mut curry, c, r, &graph);
    printer(currx, curry, &arr, c, r);
}
#[test]
fn test_scroll_left_edge_cases() {
    let mut currx = 0;
    let mut curry = 0;
    let c = 20;
    let r = 20;
    let arr = vec![Cell::default(); c * r];
    let graph = Graph::new(c * r);

    // Already at leftmost, flag should be triggered (no movement)
    assert!(scroller("s", &arr, &mut currx, &mut curry, c as i32, r as i32, &graph).is_ok());
    assert_eq!(currx, 0);

    // Move right first then test left scroll
    currx = 5;
    assert!(scroller("s", &arr, &mut currx, &mut curry, c as i32, r as i32, &graph).is_ok());
    assert_eq!(currx, 5); // 15 - 10
}
// #[test]
// fn test_scroll_right_with_column_limit() {
//     let mut currx = 0;
//     let mut curry = 0;
//     let c = 18;
//     let r = 20;
//     let arr = vec![Cell::default(); c * r];
//     let graph = Graph::new(c * r);

//     assert!(scroller("scroll_right", &arr, &mut currx, &mut curry, c, r, &graph).is_ok());
//     assert_eq!(currx, 10); // Normal jump

//     // Now currx = 10, remaining = 8
//     assert!(scroller("scroll_right", &arr, &mut currx, &mut curry, c, r, &graph).is_ok());
//     assert_eq!(currx, 18); // Should cap at 18
// }

