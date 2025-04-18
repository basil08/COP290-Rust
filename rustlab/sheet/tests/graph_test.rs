use sheet::graph::*;

#[test]
fn test_add_formula_and_edge() {
    let mut graph = Graph::new(10);
    let mut formulas = vec![Formula { op_type: -1, op_info1: -1, op_info2: -1 }; 10];

    graph.add_formula(2, 0, -1, -1, &mut formulas);
    graph.add_edge_formula(2, 10, &formulas);

    // Should have edge 2 -> 0
    let node = graph.adj_lists_head[0].as_ref().unwrap();
    assert_eq!(node.cell, 2);
}

#[test]
fn test_delete_edge() {
    let mut graph = Graph::new(10);
    let mut formulas = vec![Formula { op_type: -1, op_info1: -1, op_info2: -1 }; 10];

    graph.add_formula(2, 0, -1, -1, &mut formulas);
    graph.add_edge_formula(2, 10, &formulas);
    graph.delete_edge(2, 10, &formulas);

    assert!(graph.adj_lists_head[0].is_none());
}

#[test]
fn test_add_and_delete_range() {
    let mut graph = Graph::new(10);

    graph.add_range_to_graph(1, 3, 5);
    assert!(graph.ranges_head.is_some());

    graph.delete_range_from_graph(5);
    assert!(graph.ranges_head.is_none());
}