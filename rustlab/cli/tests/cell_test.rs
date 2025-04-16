use sheet::function::{Cell, CellValue}; // Adjust this path if needed

#[test]
fn test_new_int() {
    let cell = Cell::new_int(42);
    assert!(cell.is_valid);
    if let CellValue::Int(i) = cell.value {
        assert_eq!(i, 42);
    } else {
        panic!("Expected Int variant");
    }
}

#[test]
fn test_new_float() {
    let cell = Cell::new_float(3.14);
    assert!(cell.is_valid);
    if let CellValue::Float(f) = cell.value {
        assert!((f - 3.14).abs() < f64::EPSILON);
    } else {
        panic!("Expected Float variant");
    }
}

#[test]
fn test_new_string() {
    let cell = Cell::new_string("hello".to_string());
    assert!(cell.is_valid);
    if let CellValue::String(s) = &cell.value {
        assert_eq!(s, "hello");
    } else {
        panic!("Expected String variant");
    }
}

#[test]
fn test_invalid() {
    let cell = Cell::invalid();
    assert!(!cell.is_valid);
}

#[test]
fn test_default() {
    let cell = Cell::default();
    assert!(!cell.is_valid);
    if let CellValue::Int(i) = cell.value {
        assert_eq!(i, 0);
    } else {
        panic!("Expected Int variant with value 0");
    }
}
