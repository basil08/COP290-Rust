use sheet::parser::cell_parser;

#[test]
fn test_cell_parser() {
    // Test cases for the cell_parser function
    assert_eq!(cell_parser("A1", 10, 10, 0, 1), Ok(0));
    assert_eq!(cell_parser("B2", 10, 10, 0, 1), Ok(11));
    assert_eq!(cell_parser("C3", 10, 10, 0, 1), Ok(22));
    assert_eq!(cell_parser("D4", 10, 10, 0, 1), Ok(33));
    assert_eq!(cell_parser("E5", 10, 10, 0, 1), Ok(44));
    assert_eq!(cell_parser("AA1", 100, 100, 0, 2), Ok(26));
    assert_eq!(cell_parser("B10", 10, 20, 0, 2), Ok(91));
    assert_eq!(cell_parser("A1A", 10, 10, 0, 2), Err("Letters after digits not allowed"));
    assert_eq!(cell_parser("A#1", 10, 10, 0, 2), Err("Invalid character in cell reference"));
    assert_eq!(cell_parser("A1", 10, 10, 0, 5), Err("Invalid cell reference"));
    assert_eq!(cell_parser("Z100", 5, 5, 0, 3), Err("Cell reference out of bounds"));
    assert_eq!(cell_parser("a1", 10, 10, 0, 1), Err("Invalid character in cell reference"));
    assert_eq!(cell_parser("A1", 10, 10, 2, 1), Err("Invalid cell reference"));
}
