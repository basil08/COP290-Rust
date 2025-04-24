// use sheet::graph_ext::{Graph, Formula, State};
// use sheet::function_ext::{Cell, CellValue};

// fn setup_graph_env(size: usize) -> (Graph, Vec<Cell>, Vec<Formula>, State) {
//     let graph = Graph::new(size);
//     let arr = vec![Cell::default(); size];
//     let formulas = vec![Formula::default(); size];
//     let state = State::new();
//     (graph, arr, formulas, state)
// }

// #[test]
// fn test_add_and_delete_edge() {
//     let (mut graph, _, mut formulas, _) = setup_graph_env(10);
//     graph.add_edge(3, 5);
//     assert!(graph.adj_lists_head[5].is_some());

//     formulas[3] = Formula { op_type: 1, op_info1: 5, op_info2: 0 };
//     graph.delete_edge(3, 10, &formulas);
//     assert!(graph.adj_lists_head[5].is_none());
// }

// #[test]
// fn test_add_and_delete_range() {
//     let (mut graph, _, _, _) = setup_graph_env(10);
//     graph.add_range_to_graph(1, 3, 5);
//     assert!(graph.ranges_head.is_some());

//     graph.delete_range_from_graph(5);
//     assert!(graph.ranges_head.is_none());
// }

// #[test]
// fn test_topo_sort_and_recalc_arithmetic() {
//     let (mut graph, mut arr, mut formulas, mut state) = setup_graph_env(10);
//     arr[1] = Cell::new_int(10);
//     formulas[0] = Formula { op_type: 1, op_info1: 1, op_info2: 5 }; // =B1 + 5
//     graph.add_edge(0, 1);

//     graph.recalc(5, &mut arr, 0, &formulas, &mut state).unwrap();
//     assert_eq!(arr[0], Cell::new_int(15));
// }

// #[test]
// fn test_topo_sort_cycle_detection() {
//     let (mut graph, mut arr, mut formulas, mut state) = setup_graph_env(5);
//     formulas[0] = Formula { op_type: 1, op_info1: 1, op_info2: 0 };
//     formulas[1] = Formula { op_type: 1, op_info1: 0, op_info2: 0 };
//     graph.add_edge(0, 1);
//     graph.add_edge(1, 0);

//     let result = graph.recalc(5, &mut arr, 0, &formulas, &mut state);
//     assert!(result.is_err());
//     assert!(state.has_cycle);
// }

// #[test]
// fn test_recalc_sum() {
//     let (mut graph, mut arr, mut formulas, mut state) = setup_graph_env(25);
//     arr[6] = Cell::new_int(10);
//     arr[11] = Cell::new_int(14);
//     graph.add_range_to_graph(6, 11, 0);
//     formulas[0] = Formula { op_type: 12, op_info1: 6, op_info2: 11 }; // SUM

//     graph.recalc(5, &mut arr, 0, &formulas, &mut state).unwrap();
//     assert_eq!(arr[0], Cell::new_int(75)); // 10+11+12+13+14
// }
use sheet::function_ext::{Cell, CellValue};
use sheet::graph_ext::{Formula, Graph, State};

use sheet::parser_ext::autofill;

fn setup_graph_env(size: usize) -> (Graph, Vec<Cell>, Vec<Formula>, State) {
    let graph = Graph::new(size);
    let arr = vec![Cell::default(); size];
    let formulas = vec![Formula::default(); size];
    let state = State::new();
    (graph, arr, formulas, state)
}

#[test]
fn test_add_and_delete_edge() {
    let (mut graph, _, mut formulas, _) = setup_graph_env(10);
    graph.add_edge(3, 5);
    assert!(graph.adj_lists_head[5].is_some());

    formulas[3] = Formula {
        op_type: 1,
        op_info1: 5,
        op_info2: 0,
    };
    graph.delete_edge(3, 10, &formulas);
    assert!(graph.adj_lists_head[5].is_none());
}

#[test]
fn test_add_and_delete_range() {
    let (mut graph, _, _, _) = setup_graph_env(10);
    graph.add_range_to_graph(1, 3, 5);
    assert!(graph.ranges_head.is_some());

    graph.delete_range_from_graph(5);
    assert!(graph.ranges_head.is_none());
}
#[test]
// fn test_recalc_sum() {
//     let cell_count = 25;
//     let mut graph = Graph::new(cell_count);
//     let mut arr = vec![Cell::default(); cell_count];
//     let mut formula_array = vec![Formula::default(); cell_count];
//     let mut state = State::new();
//     state.num_cells = cell_count;

//     arr[6] = Cell::new_int(10);
//     arr[11] = Cell::new_int(14);
//     graph.add_formula(0, 6, 11, 12, &mut formula_array); // SUM
//     graph.add_range_to_graph(6, 11, 0);

//     graph.recalc(5, &mut arr, 0, &formula_array, &mut state).unwrap();
//     assert_eq!(arr[0], Cell::new_int(10 + 11 + 12 + 13 + 14));
// }
#[test]
fn test_topo_sort_and_recalc_arithmetic() {
    let cell_count = 10;
    let mut graph = Graph::new(cell_count);
    let mut arr = vec![Cell::default(); cell_count];
    let mut formula_array = vec![Formula::default(); cell_count];
    let mut state = State::new();
    state.num_cells = cell_count;

    arr[1] = Cell::new_int(5);
    graph.add_edge(0, 1);
    graph.add_formula(0, 1, 2, 1, &mut formula_array); // 5 + 2

    graph
        .recalc(5, &mut arr, 0, &formula_array, &mut state)
        .unwrap();
    assert_eq!(arr[0], Cell::new_int(7));
}

#[test]
fn test_topo_sort_cycle_detection() {
    let cell_count = 5;
    let mut graph = Graph::new(cell_count);
    let mut arr = vec![Cell::default(); cell_count];
    let mut formula_array = vec![Formula::default(); cell_count];
    let mut state = State::new();
    state.num_cells = cell_count;

    graph.add_edge(0, 1);
    graph.add_edge(1, 0); // cycle

    graph.add_formula(0, 1, 2, 5, &mut formula_array);
    graph.add_formula(1, 0, 3, 5, &mut formula_array);

    let result = graph.recalc(5, &mut arr, 0, &formula_array, &mut state);
    assert!(result.is_err());
    assert!(state.has_cycle);
}
// #[test]
// fn test_recalc_with_cell_plus_const() {
//     let mut graph = Graph::new(10);
//     let mut arr = vec![0; 10];
//     let mut formula_array = vec![Formula::default(); 10];
//     let mut has_cycle = false;

//     arr[1] = 7;
//     graph.add_formula(&mut graph, 0, 1, 3, 1, &mut formula_array); // A1 = B1 + 3
//     graph.add_edge(1, 0);

//     graph.recalc(5, &mut arr, 0, &mut formula_array, &mut has_cycle);
//     assert_eq!(arr[0], 10);
// }
#[test]
fn test_const_plus_invalid_cell() {
    let mut graph = Graph::new(5);
    let mut arr = vec![Cell::default(); 5];
    let mut formula_array = vec![Formula::default(); 5];
    let mut state = State::new();
    state.num_cells = 5;

    arr[1] = Cell::invalid(); // invalid cell
    graph.add_formula(0, 1, 5, 1, &mut formula_array); // =B1 + 5
    graph.add_edge(0, 1);

    graph
        .recalc(5, &mut arr, 0, &mut formula_array, &mut state)
        .unwrap();
    assert_eq!(arr[0], Cell::invalid());
}
#[test]
fn test_add_edge_duplicate_and_append() {
    let (mut graph, _, _, _) = setup_graph_env(10);
    graph.add_edge(3, 5); // Add edge 3 -> 5
    assert!(graph.adj_lists_head[5].is_some());
    assert_eq!(graph.adj_lists_head[5].as_ref().unwrap().cell, 3); // Line 97-98

    graph.add_edge(3, 5); // Duplicate edge, should not add (line 114)
    assert!(graph.adj_lists_head[5].as_ref().unwrap().next.is_none());

    graph.add_edge(4, 5); // Append new edge 4 -> 5 (line 107)
    assert!(graph.adj_lists_head[5].as_ref().unwrap().next.is_some());
    assert_eq!(
        graph.adj_lists_head[5]
            .as_ref()
            .unwrap()
            .next
            .as_ref()
            .unwrap()
            .cell,
        4
    );
}

#[test]
fn test_delete_node_non_head() {
    let (mut graph, _, mut formulas, _) = setup_graph_env(10);
    graph.add_edge(3, 5);
    graph.add_edge(4, 5);
    assert!(graph.adj_lists_head[5].is_some());
    assert!(graph.adj_lists_head[5].as_ref().unwrap().next.is_some());

    formulas[3] = Formula {
        op_type: 1,
        op_info1: 5,
        op_info2: 0,
    };
    graph.delete_node(4, 5); // Delete non-head node (lines 195-198)
    assert_eq!(graph.adj_lists_head[5].as_ref().unwrap().cell, 3);
    assert!(graph.adj_lists_head[5].as_ref().unwrap().next.is_none());

    graph.delete_node(5, 5); // Empty list after deletion (line 179)
    assert!(graph.adj_lists_head[5].is_some());
}

#[test]
fn test_delete_head_node() {
    let (mut graph, _, mut formulas, _) = setup_graph_env(10);
    graph.add_edge(3, 5);
    formulas[3] = Formula {
        op_type: 1,
        op_info1: 5,
        op_info2: 0,
    };
    graph.delete_node(3, 5); // Delete head node (lines 189, 191-193)
    assert!(graph.adj_lists_head[5].is_none());
}
#[test]
fn test_add_multiple_ranges() {
    let (mut graph, _, _, _) = setup_graph_env(10);
    graph.add_range_to_graph(1, 3, 5); // Lines 120-124, 126-127, 129
    graph.add_range_to_graph(2, 4, 6);
    assert!(graph.ranges_head.is_some());
    assert_eq!(graph.ranges_head.as_ref().unwrap().dependent_cell, 6);
    assert_eq!(
        graph
            .ranges_head
            .as_ref()
            .unwrap()
            .next
            .as_ref()
            .unwrap()
            .dependent_cell,
        5
    );
}

#[test]
fn test_delete_non_head_range() {
    let (mut graph, _, _, _) = setup_graph_env(10);
    graph.add_range_to_graph(1, 3, 5);
    graph.add_range_to_graph(2, 4, 6);
    graph.delete_range_from_graph(5); // Delete non-head range (lines 202-209, 211-214)
    assert!(graph.ranges_head.is_some());
    assert_eq!(graph.ranges_head.as_ref().unwrap().dependent_cell, 6);
    assert!(graph.ranges_head.as_ref().unwrap().next.is_none());

    graph.delete_range_from_graph(7); // Skip non-matching range (lines 217-218)
    assert!(graph.ranges_head.is_some());
}
#[test]
fn test_delete_edge_op_types() {
    let (mut graph, _, mut formulas, _) = setup_graph_env(10);
    // op_type = -1 (cell reference)
    formulas[0] = Formula {
        op_type: -1,
        op_info1: 1,
        op_info2: 0,
    };
    graph.add_edge(0, 1);
    graph.delete_edge(0, 5, &formulas); // Lines 221-222
    assert!(graph.adj_lists_head[1].is_none());

    // op_type = 1 (add cell + constant)
    formulas[0] = Formula {
        op_type: 1,
        op_info1: 2,
        op_info2: 0,
    };
    graph.add_edge(0, 2);
    graph.delete_edge(0, 5, &formulas); // Line 228
    assert!(graph.adj_lists_head[2].is_none());

    // op_type = 5 (add two cells)
    formulas[0] = Formula {
        op_type: 5,
        op_info1: 3,
        op_info2: 4,
    };
    graph.add_edge(0, 3);
    graph.add_edge(0, 4);
    graph.delete_edge(0, 5, &formulas); // Line 234
    assert!(graph.adj_lists_head[3].is_none());
    assert!(graph.adj_lists_head[4].is_none());

    // op_type = 9 (MIN range)
    formulas[0] = Formula {
        op_type: 9,
        op_info1: 1,
        op_info2: 3,
    };
    graph.add_range_to_graph(1, 3, 0);
    graph.delete_edge(0, 5, &formulas); // Lines 248-254
    assert!(graph.ranges_head.is_none());

    // op_type = 14 (SLEEP)
    formulas[0] = Formula {
        op_type: 14,
        op_info1: 5,
        op_info2: 0,
    };
    graph.add_edge(0, 5);
    graph.delete_edge(0, 5, &formulas); // Lines 257-258
    assert!(graph.adj_lists_head[5].is_none());

    // op_type = 15 (division constant / cell)
    formulas[0] = Formula {
        op_type: 15,
        op_info1: 0,
        op_info2: 6,
    };
    graph.add_edge(0, 6);
    graph.delete_edge(0, 5, &formulas); // Lines 266-267
    assert!(graph.adj_lists_head[6].is_none());
}

#[test]
fn test_add_edge_formula_op_types() {
    let (mut graph, _, mut formulas, _) = setup_graph_env(10);
    // op_type = -1
    formulas[0] = Formula {
        op_type: -1,
        op_info1: 1,
        op_info2: 0,
    };
    graph.add_edge_formula(0, 5, &formulas); // Lines 260-264
    assert!(graph.adj_lists_head[1].is_some());

    // op_type = 5
    formulas[0] = Formula {
        op_type: 5,
        op_info1: 2,
        op_info2: 3,
    };
    graph.add_edge_formula(0, 5, &formulas); // Lines 260-264
    assert!(graph.adj_lists_head[2].is_some());
    assert!(graph.adj_lists_head[3].is_some());

    // op_type = 9
    formulas[0] = Formula {
        op_type: 9,
        op_info1: 4,
        op_info2: 6,
    };
    graph.add_edge_formula(0, 5, &formulas); // Lines 260-264
    assert!(graph.ranges_head.is_some());

    // op_type = 14 (SLEEP with non-self reference)
    formulas[0] = Formula {
        op_type: 14,
        op_info1: 7,
        op_info2: 0,
    };
    graph.add_edge_formula(0, 5, &formulas); // Lines 260-264
    assert!(graph.adj_lists_head[7].is_some());

    // op_type = 15
    formulas[0] = Formula {
        op_type: 15,
        op_info1: 0,
        op_info2: 8,
    };
    graph.add_edge_formula(0, 5, &formulas); // Lines 260-264
    assert!(graph.adj_lists_head[8].is_some());
}
#[test]
fn test_recalc_cell_reference_invalid() {
    let (mut graph, mut arr, mut formulas, mut state) = setup_graph_env(5);
    state.num_cells = 5;
    arr[1] = Cell::invalid();
    formulas[0] = Formula {
        op_type: -1,
        op_info1: 1,
        op_info2: 0,
    };
    graph.add_edge(0, 1);

    graph.recalc(5, &mut arr, 0, &formulas, &mut state).unwrap(); // Lines 358-363
    assert_eq!(arr[0], Cell::invalid());
}

#[test]
fn test_recalc_arithmetic_cell_constant() {
    let (mut graph, mut arr, mut formulas, mut state) = setup_graph_env(5);
    state.num_cells = 5;
    arr[1] = Cell::new_int(10);
    formulas[0] = Formula {
        op_type: 3,
        op_info1: 1,
        op_info2: 2,
    }; // B1 * 2
    graph.add_edge(0, 1);

    graph.recalc(5, &mut arr, 0, &formulas, &mut state).unwrap(); // Lines 381-386
    assert_eq!(arr[0], Cell::new_int(20));

    arr[1] = Cell::invalid();
    graph.recalc(5, &mut arr, 0, &formulas, &mut state).unwrap(); // Lines 381-386
    assert_eq!(arr[0], Cell::invalid());
}

#[test]
fn test_recalc_arithmetic_two_cells() {
    let (mut graph, mut arr, mut formulas, mut state) = setup_graph_env(5);
    state.num_cells = 5;
    arr[1] = Cell::new_int(10);
    arr[2] = Cell::new_int(5);
    formulas[0] = Formula {
        op_type: 7,
        op_info1: 1,
        op_info2: 2,
    }; // B1 * C1
    graph.add_edge(0, 1);
    graph.add_edge(0, 2);

    graph.recalc(5, &mut arr, 0, &formulas, &mut state).unwrap(); // Line 402
    assert_eq!(arr[0], Cell::new_int(50));
}

#[test]
fn test_recalc_range_invalid() {
    let (mut graph, mut arr, mut formulas, mut state) = setup_graph_env(25);
    state.num_cells = 25;
    arr[6] = Cell::invalid(); // B2
    formulas[0] = Formula {
        op_type: 12,
        op_info1: 6,
        op_info2: 11,
    }; // SUM(B2:C3)
    graph.add_range_to_graph(6, 11, 0);

    graph.recalc(5, &mut arr, 0, &formulas, &mut state).unwrap(); // Line 414
    assert_eq!(arr[0], Cell::invalid());
}

#[test]
fn test_recalc_sleep() {
    let (mut graph, mut arr, mut formulas, mut state) = setup_graph_env(5);
    state.num_cells = 5;
    arr[1] = Cell::new_int(1); // Sleep for 1 second
    formulas[0] = Formula {
        op_type: 14,
        op_info1: 1,
        op_info2: 0,
    };
    graph.add_edge(0, 1);

    graph.recalc(5, &mut arr, 0, &formulas, &mut state).unwrap(); // Line 420
    assert_eq!(arr[0], Cell::new_int(1));
}

#[test]
fn test_recalc_division_constant_cell() {
    let (mut graph, mut arr, mut formulas, mut state) = setup_graph_env(5);
    state.num_cells = 5;
    arr[1] = Cell::new_int(10);
    formulas[0] = Formula {
        op_type: 15,
        op_info1: 20,
        op_info2: 1,
    }; // 20 / B1
    graph.add_edge(0, 1);

    graph.recalc(5, &mut arr, 0, &formulas, &mut state).unwrap(); // Lines 424-425, 428-429
    assert_eq!(arr[0], Cell::new_int(2));
}

#[test]
fn test_recalc_string() {
    let (mut graph, mut arr, mut formulas, mut state) = setup_graph_env(5);
    state.num_cells = 5;
    arr[0] = Cell::new_string("test".to_string());
    formulas[0] = Formula {
        op_type: 16,
        op_info1: 0,
        op_info2: 0,
    };

    graph.recalc(5, &mut arr, 0, &formulas, &mut state).unwrap(); // Lines 435-437
    assert_eq!(arr[0], Cell::new_string("test".to_string()));
}

#[test]
fn test_recalc_float() {
    let (mut graph, mut arr, mut formulas, mut state) = setup_graph_env(5);
    state.num_cells = 5;
    arr[0] = Cell::new_float(3.14);
    formulas[0] = Formula {
        op_type: 17,
        op_info1: 0,
        op_info2: 0,
    };

    graph.recalc(5, &mut arr, 0, &formulas, &mut state).unwrap(); // Lines 439-440
    assert_eq!(arr[0], Cell::new_float(3.14));
}

#[test]
fn test_recalc_invalid_op_type() {
    let (mut graph, mut arr, mut formulas, mut state) = setup_graph_env(5);
    state.num_cells = 5;
    formulas[0] = Formula {
        op_type: 999,
        op_info1: 0,
        op_info2: 0,
    }; // Invalid op_type

    graph.recalc(5, &mut arr, 0, &formulas, &mut state).unwrap(); // Line 442
    assert_eq!(arr[0], Cell::invalid());
}

// #[test]
// fn test_add_and_delete_range() {
//     let mut graph = Graph::new(5);
//     graph.add_range_to_graph(0, 1, 2);
//     graph.add_range_to_graph(3, 4, 2);
//     graph.delete_range_from_graph(2);
//     assert!(graph.ranges_head.is_none());
// }

// #[test]
// fn test_recalc_sum() {
//     let cell_count = 25;
//     let mut graph = Graph::new(cell_count);
//     let mut arr = vec![Cell::default(); cell_count];
//     let mut formula_array = vec![Formula::default(); cell_count];
//     let mut state = State::new();
//     state.num_cells = cell_count;

//     for i in 6..=11 {
//         arr[i] = Cell::new_int((i as i32) + 4);
//     }
//     graph.add_formula(0, 6, 11, 12, &mut formula_array);
//     graph.add_range_to_graph(6, 11, 0);

//     graph.recalc(5, &mut arr, 0, &formula_array, &mut state).unwrap();
//     assert_eq!(arr[0], Cell::new_int(10 + 11 + 12 + 13 + 14 + 15));
// }

// #[test]
// fn test_topo_sort_cycle_detection() {
//     let cell_count = 5;
//     let mut graph = Graph::new(cell_count);
//     let mut arr = vec![Cell::default(); cell_count];
//     let mut formula_array = vec![Formula::default(); cell_count];
//     let mut state = State::new();
//     state.num_cells = cell_count;

//     graph.add_edge(0, 1);
//     graph.add_edge(1, 0);

//     graph.add_formula(0, 1, 2, 5, &mut formula_array);
//     graph.add_formula(1, 0, 3, 5, &mut formula_array);

//     let result = graph.recalc(5, &mut arr, 0, &formula_array, &mut state);
//     assert!(result.is_err());
//     assert!(state.has_cycle);
// }

// #[test]
// fn test_string_assignment_and_recalc() {
//     let mut graph = Graph::new(10);
//     let mut arr = vec![Cell::default(); 10];
//     let mut formula_array = vec![Formula::default(); 10];
//     let mut state = State::new();
//     state.num_cells = 10;

//     arr[1] = Cell::new_string("hello".to_string());
//     graph.add_formula(0, 1, 0, -1, &mut formula_array);
//     graph.add_edge(0, 1);
//     graph.recalc(5, &mut arr, 0, &formula_array, &mut state).unwrap();
//     assert_eq!(arr[0], Cell::new_string("hello".to_string()));
// }

// #[test]
// fn test_float_assignment_and_recalc() {
//     let mut graph = Graph::new(10);
//     let mut arr = vec![Cell::default(); 10];
//     let mut formula_array = vec![Formula::default(); 10];
//     let mut state = State::new();
//     state.num_cells = 10;

//     arr[1] = Cell::new_float(3.14);
//     graph.add_formula(0, 1, 0, -1, &mut formula_array);
//     graph.add_edge(0, 1);
//     graph.recalc(5, &mut arr, 0, &formula_array, &mut state).unwrap();
//     assert_eq!(arr[0], Cell::new_float(3.14));
// }

// // #[test]
// // fn test_const_division_by_zero() {
// //     let mut graph = Graph::new(5);
// //     let mut arr = vec![Cell::default(); 5];
// //     let mut formula_array = vec![Formula::default(); 5];
// //     let mut state = State::new();
// //     state.num_cells = 5;

// //     graph.add_formula(0, 10, 0, 4, &mut formula_array);
// //     graph.recalc(5, &mut arr, 0, &mut formula_array, &mut state).unwrap();
// //     assert_eq!(arr[0], Cell::invalid());
// // }

// // #[test]
// // fn test_sleep_self_value() {
// //     let mut graph = Graph::new(5);
// //     let mut arr = vec![Cell::default(); 5];
// //     let mut formula_array = vec![Formula::default(); 5];
// //     let mut state = State::new();
// //     state.num_cells = 5;

// //     arr[0] = Cell::new_int(1);
// //     graph.add_formula(0, 0, 0, 14, &mut formula_array);
// //     graph.recalc(5, &mut arr, 0, &mut formula_array, &mut state).unwrap();
// //     assert_eq!(arr[0], Cell::new_int(1));
// // }

// #[test]
// fn test_invalid_formula_type() {
//     let mut graph = Graph::new(5);
//     let mut arr = vec![Cell::default(); 5];
//     let mut formula_array = vec![Formula::default(); 5];
//     let mut state = State::new();
//     state.num_cells = 5;

//     formula_array[0].op_type = 99;
//     graph.recalc(5, &mut arr, 0, &mut formula_array, &mut state).unwrap();
//     assert_eq!(arr[0], Cell::invalid());
// }

#[test]
fn test_minimum_value_in_range() {
    let mut graph = Graph::new(16);
    let mut arr = vec![Cell::default(); 16];
    let mut formula_array = vec![Formula::default(); 16];
    let mut state = State::new();
    state.num_cells = 16;

    arr[5] = Cell::new_int(4);
    arr[6] = Cell::new_int(9);
    arr[7] = Cell::new_int(-2);
    graph.add_formula(0, 5, 7, 9, &mut formula_array);
    graph.add_range_to_graph(5, 7, 0);

    graph
        .recalc(4, &mut arr, 0, &mut formula_array, &mut state)
        .unwrap();
    assert_eq!(arr[0], Cell::new_int(-2));
}

#[test]
fn test_division_by_zero_cell_cell() {
    let mut graph = Graph::new(5);
    let mut arr = vec![Cell::default(); 5];
    let mut formula_array = vec![Formula::default(); 5];
    let mut state = State::new();
    state.num_cells = 5;

    arr[1] = Cell::new_int(100);
    arr[2] = Cell::new_int(0);
    graph.add_formula(0, 1, 2, 8, &mut formula_array);
    graph.add_edge(0, 1);
    graph.add_edge(0, 2);
    graph
        .recalc(5, &mut arr, 0, &mut formula_array, &mut state)
        .unwrap();
    assert_eq!(arr[0], Cell::invalid());
}
