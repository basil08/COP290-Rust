use sheet::parser::cell_parser;

#[test]
fn test_cell_parser() {
    assert_eq!(cell_parser("A1", 10, 10, 0, 1).unwrap(), 0);
    assert_eq!(cell_parser("B2", 10, 10, 0, 1).unwrap(), 11);
    assert!(cell_parser("Z1", 10, 10, 0, 1).is_err());
}
