use sheet::{generate_grid, CellValue};

#[test]
fn test_generate_grid_initialization() {
    let rows = 3;
    let cols = 4;
    let grid = generate_grid(rows, cols);

    assert_eq!(grid.len(), rows, "Grid should have correct number of rows");
    for row in &grid {
        assert_eq!(row.len(), cols, "Each row should have correct number of columns");
        for cell in row {
            assert!(cell.is_valid, "Each cell should be valid");
            assert_eq!(cell.value, CellValue::Int(0), "Each cell should be initialized to Int(0)");
        }
    }
}
