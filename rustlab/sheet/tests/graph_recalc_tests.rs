use sheet::graph::{Formula, Graph};
use std::i32;

fn setup(cells: usize) -> (Graph, Vec<i32>, Vec<Formula>, bool) {
    (Graph::new(cells), vec![0; cells], vec![Formula::default(); cells], false)
}
#[test]
fn test_add_duplicate_edge() {
    let (mut graph, mut arr, mut formulas, mut has_cycle) = setup(5);
    graph.add_edge(0, 1); // Add edge 0->1
    graph.add_edge(0, 1); // Duplicate edge (lines 75-78)
    assert!(graph.has_edge(0, 1)); // Lines 85-86, 90
    let mut current = &graph.adj_lists[0];
    let mut count = 0;
    while let Some(cell) = current {
        count += 1;
        current = &cell.next;
    }
    assert_eq!(count, 1); // Only one edge
}

#[test]
fn test_add_and_delete_multiple_edges() {
    let (mut graph, mut arr, mut formulas, mut has_cycle) = setup(5);
    graph.add_edge(0, 1); // Lines 40-41, 80-83
    graph.add_edge(0, 2);
    assert!(graph.has_edge(0, 1)); // Lines 85-86, 90
    assert!(graph.has_edge(0, 2));
    graph.delete_edge(0, 1); // Lines 96, 108
    assert!(!graph.has_edge(0, 1));
    assert!(graph.has_edge(0, 2));
}
#[test]
fn test_add_and_delete_multiple_ranges() {
    let (mut graph, mut arr, mut formulas, mut has_cycle) = setup(5);
    graph.add_range_to_graph(0, 1, 2); // Lines 67, 70, 125, 128
    graph.add_range_to_graph(3, 4, 2);
    let mut count = 0;
    let mut current = &graph.ranges;
    while let Some(_) = current {
        count += 1;
        current = &current.as_ref().unwrap().next;
    }
    assert_eq!(count, 2);
    graph.delete_range(2); // Lines 143, 151-156, 158
    assert!(graph.ranges.is_none());
}
#[test]
fn test_arithmetic_eval_invalid_operator() {
    let result = Graph::arithmetic_eval2(5, 2, '^'); // Lines 163-168
    assert_eq!(result, i32::MIN);
}

#[test]
fn test_add_formula_constant() {
    let (mut graph, mut arr, mut formulas, mut has_cycle) = setup(5);
    Graph::add_formula(&mut graph, 0, 42, 0, 0, &mut formulas); // Line 172
    assert_eq!(formulas[0].op_type, 0);
    assert_eq!(formulas[0].op_info1, 42);
}
#[test]
fn test_topo_sort_with_cycle_and_ranges() {
    let (mut graph, mut arr, mut formulas, mut has_cycle) = setup(25);
    graph.add_edge(0, 1);
    graph.add_edge(1, 0); // Cycle
    graph.add_range_to_graph(5, 10, 0); // Lines 215, 219-222, 224
    let mut visited = vec![false; 25];
    let mut on_stack = vec![false; 25];
    let mut stack = Vec::new();
    graph.topo_sort_from_cell(
        0,
        5,
        &mut visited,
        &mut on_stack,
        &mut stack,
        &formulas,
        &mut has_cycle,
    ); // Lines 205-206
    assert!(has_cycle);
}
#[test]
fn test_recalc_constant_assignment() {
    let (mut graph, mut arr, mut formulas, mut has_cycle) = setup(5);
    Graph::add_formula(&mut graph, 0, 42, 0, 0, &mut formulas); // Lines 235-236
    graph.recalc(5, &mut arr, 0, &mut formulas, &mut has_cycle);
    assert_eq!(arr[0], 42);
}

// #[test]
// fn test_recalc_constant_division_by_zero() {
//     let (mut graph, mut arr, mut formulas, mut has_cycle) = setup(5);
//     Graph::add_formula(&mut graph, 0, 10, 0, 4, &mut formulas); // Lines 238-239, 241
//     graph.recalc(5, &mut arr, 0, &mut formulas, &mut has_cycle);
//     assert_eq!(arr[0], i32::MIN);
// }

#[test]
fn test_recalc_cell_cell_subtraction() {
    let (mut graph, mut arr, mut formulas, mut has_cycle) = setup(5);
    arr[1] = 10;
    arr[2] = 3;
    Graph::add_formula(&mut graph, 0, 1, 2, 6, &mut formulas); // Lines 255-256
    graph.add_edge(0, 1);
    graph.add_edge(0, 2);
    graph.recalc(5, &mut arr, 0, &mut formulas, &mut has_cycle);
    assert_eq!(arr[0], 7);
}

#[test]
fn test_recalc_cell_cell_division_by_zero() {
    let (mut graph, mut arr, mut formulas, mut has_cycle) = setup(5);
    arr[1] = 100;
    arr[2] = 0;
    Graph::add_formula(&mut graph, 0, 1, 2, 8, &mut formulas); // Lines 255-256, 260-261
    graph.add_edge(0, 1);
    graph.add_edge(0, 2);
    graph.recalc(5, &mut arr, 0, &mut formulas, &mut has_cycle);
    assert_eq!(arr[0], i32::MIN);
}

#[test]
fn test_recalc_max_range() {
    let (mut graph, mut arr, mut formulas, mut has_cycle) = setup(16);
    arr[5] = 4;
    arr[6] = 9;
    arr[7] = -2;
    Graph::add_formula(&mut graph, 0, 5, 7, 10, &mut formulas); // Lines 266-271, 291
    graph.add_range_to_graph(5, 7, 0);
    graph.recalc(4, &mut arr, 0, &mut formulas, &mut has_cycle);
    assert_eq!(arr[0], 9); // MAX
}

#[test]
fn test_recalc_avg_range() {
    let (mut graph, mut arr, mut formulas, mut has_cycle) = setup(16);
    arr[5] = 4;
    arr[6] = 9;
    arr[7] = 6;
    Graph::add_formula(&mut graph, 0, 5, 7, 11, &mut formulas); // Lines 266-271, 296
    graph.add_range_to_graph(5, 7, 0);
    graph.recalc(4, &mut arr, 0, &mut formulas, &mut has_cycle);
    assert_eq!(arr[0], 6); // AVG = (4+9+6)/3
}

#[test]
fn test_recalc_stdev_range() {
    let (mut graph, mut arr, mut formulas, mut has_cycle) = setup(16);
    arr[5] = 2;
    arr[6] = 4;
    arr[7] = 6;
    Graph::add_formula(&mut graph, 0, 5, 7, 13, &mut formulas); // Lines 266-271, 299-309
    graph.add_range_to_graph(5, 7, 0);
    graph.recalc(4, &mut arr, 0, &mut formulas, &mut has_cycle);
    assert_eq!(arr[0], 2); // STDEV ≈ 2 (std dev of [2,4,6])
}

// #[test]
// fn test_recalc_empty_range() {
//     let (mut graph, mut arr, mut formulas, mut has_cycle) = setup(16);
//     Graph::add_formula(&mut graph, 0, 5, 5, 12, &mut formulas); // Lines 266-271, 280-281
//     graph.add_range_to_graph(5, 5, 0);
//     graph.recalc(4, &mut arr, 0, &mut formulas, &mut has_cycle);
//     assert_eq!(arr[0], i32::MIN);
// }

#[test]
fn test_recalc_sleep_self_reference() {
    let (mut graph, mut arr, mut formulas, mut has_cycle) = setup(5);
    arr[0] = 1;
    Graph::add_formula(&mut graph, 0, 0, 1, 14, &mut formulas); // Lines 322-323, 337
    graph.recalc(5, &mut arr, 0, &mut formulas, &mut has_cycle);
    assert_eq!(arr[0], 1);
}

#[test]
fn test_recalc_constant_cell_division() {
    let (mut graph, mut arr, mut formulas, mut has_cycle) = setup(5);
    arr[1] = 2;
    Graph::add_formula(&mut graph, 0, 10, 1, 15, &mut formulas); // Lines 344-345, 353, 355-356
    graph.add_edge(0, 1);
    graph.recalc(5, &mut arr, 0, &mut formulas, &mut has_cycle);
    assert_eq!(arr[0], 5); // 10/2
}

#[test]
fn test_recalc_invalid_formula_type() {
    let (mut graph, mut arr, mut formulas, mut has_cycle) = setup(5);
    formulas[0] = Formula { op_type: 99, op_info1: 0, op_info2: 0 }; // Lines 362, 364-367
    graph.recalc(5, &mut arr, 0, &mut formulas, &mut has_cycle);
    assert_eq!(arr[0], i32::MIN);
}
