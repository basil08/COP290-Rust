use sheet::function::*;
use sheet::graph::{Formula, Graph};

fn setup_grid() -> (Vec<i32>, Graph, Vec<Formula>) {
    let mut arr = vec![0; 25]; // 5x5 grid
                               // Fill B2 and B3 with values: B2(6), B3(8)
    arr[1 + 1 * 5] = 6; // B2
    arr[1 + 2 * 5] = 8; // B3
    let graph = Graph::new(25);
    let formula_array = vec![Formula::default(); 25];
    (arr, graph, formula_array)
}

#[test]
fn test_min_func() {
    let (mut arr, mut graph, mut formulas) = setup_grid();
    let result = min_func("A1=min(B2:B3)", 5, 5, 3, 0, &mut arr, &mut graph, &mut formulas);
    assert_eq!(result, 1);
    assert_eq!(arr[0], 6);
}

#[test]
fn test_max_func() {
    let (mut arr, mut graph, mut formulas) = setup_grid();
    let result = max_func("A1=max(B2:B3)", 5, 5, 3, 0, &mut arr, &mut graph, &mut formulas);
    assert_eq!(result, 1);
    assert_eq!(arr[0], 8);
}

#[test]
fn test_sum_func() {
    let (mut arr, mut graph, mut formulas) = setup_grid();
    let result = sum_func("A1=sum(B2:B3)", 5, 5, 3, 0, &mut arr, &mut graph, &mut formulas);
    assert_eq!(result, 1);
    assert_eq!(arr[0], 6 + 8);
}

#[test]
fn test_avg_func() {
    let (mut arr, mut graph, mut formulas) = setup_grid();
    let result = avg_func("A1=avg(B2:B3)", 5, 5, 3, 0, &mut arr, &mut graph, &mut formulas);
    assert_eq!(result, 1);
    assert_eq!(arr[0], (6 + 8) / 2);
}

#[test]
fn test_stdev_func() {
    let (mut arr, mut graph, mut formulas) = setup_grid();
    let result = stdev_func("A1=stdev(B2:B3)", 5, 5, 3, 0, &mut arr, &mut graph, &mut formulas);
    assert_eq!(result, 1);
    // stddev of [6, 8] = sqrt(1) = 1
    assert_eq!(arr[0], 1);
}

#[test]
fn test_sleep_func_with_value() {
    let mut arr = vec![0; 25];
    let mut graph = Graph::new(25);
    let mut formulas = vec![Formula::default(); 25];
    let result = sleep_func("A1=sleep(5)", 5, 5, 3, 0, &mut arr, &mut graph, &mut formulas);
    assert_eq!(result, 1);
    assert_eq!(arr[0], 5);
}

#[test]
fn test_sleep_func_with_reference() {
    let mut arr = vec![0; 25];
    arr[1 + 1 * 5] = 4; // B2 = 4
    let mut graph = Graph::new(25);
    let mut formulas = vec![Formula::default(); 25];
    let result = sleep_func("A1=sleep(B2)", 5, 5, 3, 0, &mut arr, &mut graph, &mut formulas);
    assert_eq!(result, 1);
    assert_eq!(arr[0], 4);
}
