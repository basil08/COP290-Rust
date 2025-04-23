use sheet::function_ext::{Cell, CellValue};

#[test]
fn test_new_int_cell() {
    let cell = Cell::new_int(100);
    assert_eq!(cell.value, CellValue::Int(100));
    assert!(cell.is_valid);
}

#[test]
fn test_new_float_cell() {
    let cell = Cell::new_float(12.34);
    assert_eq!(cell.value, CellValue::Float(12.34));
    assert!(cell.is_valid);
}

#[test]
fn test_new_string_cell() {
    let cell = Cell::new_string("hello".to_string());
    assert_eq!(cell.value, CellValue::String("hello".to_string()));
    assert!(cell.is_valid);
}

#[test]
fn test_invalid_cell() {
    let cell = Cell::invalid();
    assert_eq!(cell.value, CellValue::Int(0));
    assert!(!cell.is_valid);
}

#[test]
fn test_default_cell() {
    let default: Cell = Default::default();
    assert_eq!(default, Cell::invalid());
} 
