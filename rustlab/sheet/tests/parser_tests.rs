use sheet::graph::{Formula, Graph};
use std::i32;

fn setup(cells: usize) -> (Graph, Vec<i32>, Vec<Formula>) {
    (
        Graph::new(cells),
        vec![0; cells],
        vec![Formula::default(); cells],
    )
}

#[test]
fn test_parser_const_assignment() {
    let (mut graph, mut arr, mut formulas) = setup(5);
    let status = sheet::parser::parser("A1=42", 5, 1, &mut arr, &mut graph, &mut formulas);
    assert_eq!(status, 1);
    assert_eq!(arr[0], 42);
}

#[test]
fn test_parser_cell_reference() {
    let (mut graph, mut arr, mut formulas) = setup(5);
    arr[1] = 30; // B1
    let status = sheet::parser::parser("A1=B1", 5, 1, &mut arr, &mut graph, &mut formulas);
    assert_eq!(status, 1);
    assert_eq!(arr[0], 30);
}

#[test]
fn test_parser_arithmetic_const_expr() {
    let (mut graph, mut arr, mut formulas) = setup(5);
    let status = sheet::parser::parser("A1=3+7", 5, 1, &mut arr, &mut graph, &mut formulas);
    assert_eq!(status, 1);
    assert_eq!(arr[0], 10);
}

#[test]
fn test_parser_arithmetic_cell_const_expr() {
    let (mut graph, mut arr, mut formulas) = setup(5);
    arr[1] = 9; // B1
    let status = sheet::parser::parser("A1=B1+6", 5, 1, &mut arr, &mut graph, &mut formulas);
    assert_eq!(status, 1);
    assert_eq!(arr[0], 15);
}

#[test]
fn test_parser_arithmetic_const_cell_expr() {
    let (mut graph, mut arr, mut formulas) = setup(5);
    arr[2] = 3; // C1
    let status = sheet::parser::parser("A1=6/C1", 5, 1, &mut arr, &mut graph, &mut formulas);
    assert_eq!(status, 1);
    assert_eq!(arr[0], 2);
}

#[test]
fn test_parser_arithmetic_cell_cell_expr_multiplication() {
    let (mut graph, mut arr, mut formulas) = setup(5);
    arr[1] = 5; // B1
    arr[2] = 2; // C1
    let status = sheet::parser::parser("A1=B1*C1", 5, 1, &mut arr, &mut graph, &mut formulas);
    assert_eq!(status, 1);
    assert_eq!(arr[0], 10);
}

#[test]
fn test_parser_arithmetic_cell_cell_expr_addition() {
    let (mut graph, mut arr, mut formulas) = setup(5);
    arr[1] = 5; // B1
    arr[2] = 2; // C1
    let status = sheet::parser::parser("A1=B1+C1", 5, 1, &mut arr, &mut graph, &mut formulas);
    assert_eq!(status, 1);
    assert_eq!(arr[0], 7);
}

#[test]
fn test_parser_arithmetic_cell_cell_expr_subtraction() {
    let (mut graph, mut arr, mut formulas) = setup(5);
    arr[1] = 5; // B1
    arr[2] = 2; // C1
    let status = sheet::parser::parser("A1=B1-C1", 5, 1, &mut arr, &mut graph, &mut formulas);
    assert_eq!(status, 1);
    assert_eq!(arr[0], 3);
}



#[test]
fn test_parser_arithmetic_cell_cell_expr_division() {
    let (mut graph, mut arr, mut formulas) = setup(5);
    arr[1] = 10; // B1
    arr[2] = 2; // C1
    let status = sheet::parser::parser("A1=B1/C1", 5, 1, &mut arr, &mut graph, &mut formulas);
    assert_eq!(status, 1);
    assert_eq!(arr[0], 5);
}

#[test]
fn test_parser_arithmetic_cell_negative_constant() {
    let (mut graph, mut arr, mut formulas) = setup(5);
    let status = sheet::parser::parser("A1=-10", 5, 1, &mut arr, &mut graph, &mut formulas);
    assert_eq!(status, 1);
    assert_eq!(arr[0], -10);
}

#[test]
fn test_parser_arithmetic_const_addition() {
    let (mut graph, mut arr, mut formulas) = setup(5);
    let status = sheet::parser::parser("A1=10+5", 5, 1, &mut arr, &mut graph, &mut formulas);
    assert_eq!(status, 1);
    assert_eq!(arr[0], 15);
}

#[test]
fn test_parser_arithmetic_const_subtraction() {
    let (mut graph, mut arr, mut formulas) = setup(5);
    let status = sheet::parser::parser("A1=10-5", 5, 1, &mut arr, &mut graph, &mut formulas);
    assert_eq!(status, 1);
    assert_eq!(arr[0], 5);
}

#[test]
fn test_parser_arithmetic_const_multiplication() {
    let (mut graph, mut arr, mut formulas) = setup(5);
    let status = sheet::parser::parser("A1=10*5", 5, 1, &mut arr, &mut graph, &mut formulas);
    assert_eq!(status, 1);
    assert_eq!(arr[0], 50);
}

#[test]
fn test_parser_arithmetic_const_division() {
    let (mut graph, mut arr, mut formulas) = setup(5);
    let status = sheet::parser::parser("A1=10/5", 5, 1, &mut arr, &mut graph, &mut formulas);
    assert_eq!(status, 1);
    assert_eq!(arr[0], 2);
}

#[test]
fn test_parser_arithmetic_cell_unknown_operator() {
    let (mut graph, mut arr, mut formulas) = setup(5);
    let status = sheet::parser::parser("A1=10@5", 5, 1, &mut arr, &mut graph, &mut formulas);
    assert_eq!(status, -1);
}

#[test]
fn test_parser_arithmetic_divide_by_zero() {
    let (mut graph, mut arr, mut formulas) = setup(5);
    let status = sheet::parser::parser("A1=10/0", 5, 1, &mut arr, &mut graph, &mut formulas);
    assert_eq!(status, 1);
    assert_eq!(arr[0], i32::MIN);
}

#[test]
fn test_parser_sleep_with_value() {
    let (mut graph, mut arr, mut formulas) = setup(5);
    let status = sheet::parser::parser("A1=SLEEP(0)", 5, 1, &mut arr, &mut graph, &mut formulas);
    assert_eq!(status, 1);
    assert_eq!(arr[0], 0);
}

#[test]
fn test_parser_sleep_with_cell_ref() {
    let (mut graph, mut arr, mut formulas) = setup(5);
    arr[1] = 3; // B1
    let status = sheet::parser::parser("A1=SLEEP(B1)", 5, 1, &mut arr, &mut graph, &mut formulas);
    assert_eq!(status, 1);
    assert_eq!(arr[0], 3);
}

#[test]
fn test_parser_invalid_sleep_format() {
    let (mut graph, mut arr, mut formulas) = setup(5);
    let status = sheet::parser::parser("A1=SLEEP(1", 5, 1, &mut arr, &mut graph, &mut formulas);
    assert_eq!(status, -1);
}

#[test]
fn test_parser_invalid_cell_name() {
    let (mut graph, mut arr, mut formulas) = setup(5);
    let status = sheet::parser::parser("A1=XX", 5, 1, &mut arr, &mut graph, &mut formulas);
    assert_eq!(status, -1);
}

#[test]
fn test_parser_invalid_syntax() {
    let (mut graph, mut arr, mut formulas) = setup(5);
    let status = sheet::parser::parser("=42", 5, 1, &mut arr, &mut graph, &mut formulas);
    assert_eq!(status, -1);
}

// #[test]
// fn test_formula_const_div_by_zero() {
//     let (mut graph, mut arr, mut formulas) = setup(5);
//     Graph::add_formula(&mut graph, 0, 10, 0, 4, &mut formulas);
//     let mut has_cycle = false;
//     graph.recalc(5, &mut arr, 0, &mut formulas, &mut has_cycle);
//     assert_eq!(arr[0], i32::MIN);
// }

#[test]
fn test_topo_sort_with_range_dependencies() {
    let (mut graph, mut arr, mut formulas) = setup(25);
    arr[6] = 10;
    arr[11] = 14;
    Graph::add_formula(&mut graph, 0, 6, 11, 12, &mut formulas);
    graph.add_range_to_graph(6, 11, 0);
    graph.recalc(5, &mut arr, 0, &mut formulas, &mut false);
    assert_eq!(arr[0], 24);
}
// #[test]
// fn test_parser_lowercase_cell_name() {
//     let (mut graph, mut arr, mut formulas) = setup(5);
//     let status = sheet::parser::parser("a1=42", 5, 1, &mut arr, &mut graph, &mut formulas); // Lines 11, 84-86, 100
//     assert_eq!(status, -1);
// }

#[test]
fn test_parser_invalid_char_in_cell() {
    let (mut graph, mut arr, mut formulas) = setup(5);
    let status = sheet::parser::parser("A@=42", 5, 1, &mut arr, &mut graph, &mut formulas); // Lines 84-86, 100
    assert_eq!(status, -1);
}

#[test]
fn test_parser_letter_after_digit() {
    let (mut graph, mut arr, mut formulas) = setup(5);
    let status = sheet::parser::parser("A1B=42", 5, 1, &mut arr, &mut graph, &mut formulas); // Lines 84-86
    assert_eq!(status, -1);
}

#[test]
fn test_parser_multi_digit_row() {
    let (mut graph, mut arr, mut formulas) = setup(25);
    arr[5 * 2] = 99; // A3 (row 3, index 10)
    let status = sheet::parser::parser("A1=A3", 5, 5, &mut arr, &mut graph, &mut formulas); // Lines 96, 98
    assert_eq!(status, 1);
    assert_eq!(arr[0], 99);
}

#[test]
fn test_parser_out_of_bounds_cell() {
    let (mut graph, mut arr, mut formulas) = setup(5);
    let status = sheet::parser::parser("F1=42", 5, 1, &mut arr, &mut graph, &mut formulas); // Lines 113-117
    assert_eq!(status, -1);
}

#[test]
fn test_parser_negative_row() {
    let (mut graph, mut arr, mut formulas) = setup(5);
    let status = sheet::parser::parser("A0=42", 5, 1, &mut arr, &mut graph, &mut formulas); // Lines 113-117
    assert_eq!(status, -1);
}

#[test]
fn test_parser_invalid_range() {
    let (mut graph, mut arr, mut formulas) = setup(5);
    let status = sheet::parser::parser("A1=A2", 5, 1, &mut arr, &mut graph, &mut formulas); // Lines 41, 61, 131, 133-136
    assert_eq!(status, -1); // A2 out of bounds (row=2 > r=1)
}
#[test]
fn test_parser_empty_input() {
    let (mut graph, mut arr, mut formulas) = setup(5);
    let status = sheet::parser::parser("", 5, 1, &mut arr, &mut graph, &mut formulas); // Lines 166-167
    assert_eq!(status, -1);
}

#[test]
fn test_parser_no_equals_sign() {
    let (mut graph, mut arr, mut formulas) = setup(5);
    let status = sheet::parser::parser("A1 42", 5, 1, &mut arr, &mut graph, &mut formulas); // Lines 169-170, 183
    assert_eq!(status, -1);
}

// #[test]
// fn test_parser_positive_constant() {
//     let (mut graph, mut arr, mut formulas) = setup(5);
//     let status = sheet::parser::parser("A1=+42", 5, 1, &mut arr, &mut graph, &mut formulas); // Lines 205, 209-210, 212, 216, 221

// }

// #[test]
// fn test_parser_negative_cell_reference() {
//     let (mut graph, mut arr, mut formulas) = setup(5);
//     arr[1] = 10; // B1
//     let status = sheet::parser::parser("A1=-B1", 5, 1, &mut arr, &mut graph, &mut formulas); // Lines 205, 207, 209, 226, 230, 233
//     assert_eq!(status, 1);
//     assert_eq!(arr[0], -10);
// }

#[test]
fn test_parser_arithmetic_subtraction() {
    let (mut graph, mut arr, mut formulas) = setup(5);
    let status = sheet::parser::parser("A1=10-3", 5, 1, &mut arr, &mut graph, &mut formulas); // Lines 241, 243-246, 249
    assert_eq!(status, 1);
    assert_eq!(arr[0], 7);
}

#[test]
fn test_parser_arithmetic_multiplication() {
    let (mut graph, mut arr, mut formulas) = setup(5);
    let status = sheet::parser::parser("A1=4*5", 5, 1, &mut arr, &mut graph, &mut formulas); // Lines 241, 243-246, 253-254
    assert_eq!(status, 1);
    assert_eq!(arr[0], 20);
}

#[test]
fn test_parser_invalid_operator() {
    let (mut graph, mut arr, mut formulas) = setup(5);
    let status = sheet::parser::parser("A1=2^3", 5, 1, &mut arr, &mut graph, &mut formulas); // Lines 241, 243-246, 258-259
    assert_eq!(status, -1);
}

#[test]
fn test_parser_sleep_missing_paren() {
    let (mut graph, mut arr, mut formulas) = setup(5);
    let status = sheet::parser::parser("A1=SLEEP(1", 5, 1, &mut arr, &mut graph, &mut formulas); // Lines 261-263
    assert_eq!(status, -1);
}

#[test]
fn test_parser_unknown_function() {
    let (mut graph, mut arr, mut formulas) = setup(5);
    let status = sheet::parser::parser("A1=XYZ(1)", 5, 1, &mut arr, &mut graph, &mut formulas); // Lines 261, 267-268
    assert_eq!(status, -1);
}
#[test]
fn test_parser_sum_missing_colon() {
    let (mut graph, mut arr, mut formulas) = setup(5);
    let status = sheet::parser::parser("A1=SUM(B1 C1)", 5, 1, &mut arr, &mut graph, &mut formulas); // Lines 289-293
    assert_eq!(status, -1);
}

#[test]
fn test_parser_sum_invalid_range() {
    let (mut graph, mut arr, mut formulas) = setup(5);
    let status = sheet::parser::parser("A1=SUM(B1:Z1)", 5, 1, &mut arr, &mut graph, &mut formulas); // Lines 294-296
    assert_eq!(status, -1);
}

#[test]
fn test_parser_sum_empty_range() {
    let (mut graph, mut arr, mut formulas) = setup(5);
    let status = sheet::parser::parser("A1=SUM(B1:B1)", 5, 1, &mut arr, &mut graph, &mut formulas); // Lines 289-296, 300-301
    assert_eq!(status, 1);
    assert_eq!(arr[0], 0);
}

#[test]
fn test_parser_valid_sum_range() {
    let (mut graph, mut arr, mut formulas) = setup(5);
    arr[1] = 10;
    arr[2] = 20;
    let status = sheet::parser::parser("A1=SUM(B1:C1)", 5, 1, &mut arr, &mut graph, &mut formulas);
    assert_eq!(status, 1);
    assert_eq!(arr[0], 30);
}

#[test]
fn test_parser_valid_stdev_range() {
    let (mut graph, mut arr, mut formulas) = setup(5);
    arr[1] = 10;
    arr[2] = 20;
    let status = sheet::parser::parser("A1=STDEV(B1:C1)", 5, 1, &mut arr, &mut graph, &mut formulas);
    assert_eq!(status, 1);
}

#[test]
fn test_parser_valid_average_range() {
    let (mut graph, mut arr, mut formulas) = setup(5);
    arr[1] = 10;
    arr[2] = 20;
    let status = sheet::parser::parser("A1=AVG(B1:C1)", 5, 1, &mut arr, &mut graph, &mut formulas);
    assert_eq!(status, 1);
}

#[test]
fn test_parser_valid_min_range() {
    let (mut graph, mut arr, mut formulas) = setup(5);
    arr[1] = 10;
    arr[2] = 20;
    let status = sheet::parser::parser("A1=MIN(B1:C1)", 5, 1, &mut arr, &mut graph, &mut formulas);
    assert_eq!(status, 1);
}

#[test]
fn test_parser_valid_max_range() {
    let (mut graph, mut arr, mut formulas) = setup(5);
    arr[1] = 10;
    arr[2] = 20;
    let status = sheet::parser::parser("A1=MAX(B1:C1)", 5, 1, &mut arr, &mut graph, &mut formulas);
    assert_eq!(status, 1);
}

// #[test]
// fn test_parser_cycle_in_range() {
//     let (mut graph, mut arr, mut formulas) = setup(25);
//     arr[6] = 10; // B2
//     arr[11] = 14; // B3
//     Graph::add_formula(&mut graph, 6, 0, 0, -1, &mut formulas); // B2 = A1
//     graph.add_edge(6, 0);
//     let status = sheet::parser::parser("A1=SUM(B2:B3)", 5, 5, &mut arr, &mut graph, &mut formulas); // Lines 304-310
//     assert_eq!(status, -1);
//     assert_eq!(arr[0], 0); // Cycle resets A1
// }
