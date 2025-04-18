use sheet::util::{arithmetic_eval, return_optype};
use sheet::function::{Cell, CellValue};
#[test]
fn test_int_operations() {
    let a = Cell::new_int(10);
    let b = Cell::new_int(2);
    let zero = Cell::new_int(0);

    assert_eq!(arithmetic_eval(a.clone(), b.clone(), '+').value, CellValue::Int(12));
    assert_eq!(arithmetic_eval(a.clone(), b.clone(), '-').value, CellValue::Int(8));
    assert_eq!(arithmetic_eval(a.clone(), b.clone(), '*').value, CellValue::Int(20));
    assert_eq!(arithmetic_eval(a.clone(), b.clone(), '/').value, CellValue::Int(5));
    assert!(!arithmetic_eval(a.clone(), zero.clone(), '/').is_valid);
    assert_eq!(arithmetic_eval(Cell::new_int(7), Cell::new_int(2), '/').value, CellValue::Float(3.5));
}

#[test]
fn test_float_operations() {
    let a = Cell::new_float(10.5);
    let b = Cell::new_float(2.0);
    let zero = Cell::new_float(0.0);

    assert_eq!(arithmetic_eval(a.clone(), b.clone(), '+').value, CellValue::Float(12.5));
    assert_eq!(arithmetic_eval(a.clone(), b.clone(), '-').value, CellValue::Float(8.5));
    assert_eq!(arithmetic_eval(a.clone(), b.clone(), '*').value, CellValue::Float(21.0));
    assert_eq!(arithmetic_eval(a.clone(), b.clone(), '/').value, CellValue::Float(5.25));
    assert!(!arithmetic_eval(a.clone(), zero.clone(), '/').is_valid);
}

#[test]
fn test_mixed_operations() {
    let i = Cell::new_int(10);
    let f = Cell::new_float(2.5);

    assert_eq!(arithmetic_eval(i.clone(), f.clone(), '+').value, CellValue::Float(12.5));
    assert_eq!(arithmetic_eval(i.clone(), f.clone(), '-').value, CellValue::Float(7.5));
    assert_eq!(arithmetic_eval(i.clone(), f.clone(), '*').value, CellValue::Float(25.0));
    assert_eq!(arithmetic_eval(i.clone(), f.clone(), '/').value, CellValue::Float(4.0));

    let i2 = Cell::new_int(5);
    let f2 = Cell::new_float(2.0);
    assert_eq!(arithmetic_eval(f2.clone(), i2.clone(), '+').value, CellValue::Float(7.0));
    assert_eq!(arithmetic_eval(f2.clone(), i2.clone(), '/').value, CellValue::Float(0.4));
}

#[test]
fn test_string_operations() {
    let s1 = Cell::new_string("Hello".to_string());
    let s2 = Cell::new_string("World".to_string());

    assert_eq!(
        arithmetic_eval(s1.clone(), s2.clone(), '+').value,
        CellValue::String("HelloWorld".to_string())
    );
    assert!(!arithmetic_eval(s1.clone(), s2.clone(), '-').is_valid);
    assert!(!arithmetic_eval(s1.clone(), s2.clone(), '*').is_valid);
    assert!(!arithmetic_eval(s1.clone(), s2.clone(), '/').is_valid);
}

#[test]
fn test_invalid_combinations() {
    let int_cell = Cell::new_int(5);
    let str_cell = Cell::new_string("Test".to_string());

    assert!(!arithmetic_eval(int_cell.clone(), str_cell.clone(), '+').is_valid);
    assert!(!arithmetic_eval(str_cell.clone(), int_cell.clone(), '*').is_valid);
}

#[test]
fn test_invalid_cells() {
    let valid = Cell::new_int(5);
    let invalid = Cell::invalid();

    assert!(!arithmetic_eval(valid.clone(), invalid.clone(), '+').is_valid);
    assert!(!arithmetic_eval(invalid.clone(), valid.clone(), '-').is_valid);
    assert!(!arithmetic_eval(invalid.clone(), invalid.clone(), '*').is_valid);
}

#[test]
fn test_unknown_operator() {
    let a = Cell::new_int(2);
    let b = Cell::new_int(3);

    assert!(!arithmetic_eval(a, b, '%').is_valid);
}

#[test]
fn test_return_optype_values() {
    assert_eq!(return_optype('+'), 1);
    assert_eq!(return_optype('-'), 2);
    assert_eq!(return_optype('*'), 3);
    assert_eq!(return_optype('/'), 4);
    assert_eq!(return_optype('%'), -1);
    assert_eq!(return_optype('^'), -1);
}

