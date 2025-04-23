use sheet::function_ext::{Cell, CellValue};
use sheet::util_ext::{arithmetic_eval, return_optype};

#[test]
fn test_arithmetic_eval_int_sub() {
    let c1 = Cell::new_int(10);
    let c2 = Cell::new_int(3);
    let result = arithmetic_eval(c1, c2, '-');
    assert_eq!(result.value, CellValue::Int(7));
    assert!(result.is_valid);
}

#[test]
fn test_arithmetic_eval_int_div_exact() {
    let c1 = Cell::new_int(8);
    let c2 = Cell::new_int(2);
    let result = arithmetic_eval(c1, c2, '/');
    assert_eq!(result.value, CellValue::Int(4));
    assert!(result.is_valid);
}

#[test]
fn test_arithmetic_eval_int_div_non_exact() {
    let c1 = Cell::new_int(7);
    let c2 = Cell::new_int(2);
    let result = arithmetic_eval(c1, c2, '/');
    assert_eq!(result.value, CellValue::Float(3.5));
    assert!(result.is_valid);
}

#[test]
fn test_arithmetic_eval_float_div_by_zero() {
    let c1 = Cell::new_float(5.0);
    let c2 = Cell::new_float(0.0);
    let result = arithmetic_eval(c1, c2, '/');
    assert!(!result.is_valid);
}

#[test]
fn test_arithmetic_eval_mixed_float_int_add() {
    let c1 = Cell::new_int(4);
    let c2 = Cell::new_float(3.5);
    let result = arithmetic_eval(c1, c2, '+');
    assert_eq!(result.value, CellValue::Float(7.5));
    assert!(result.is_valid);
}

#[test]
fn test_arithmetic_eval_invalid_cell() {
    let mut c1 = Cell::new_int(5);
    let c2 = Cell::new_int(5);
    c1.is_valid = false;
    let result = arithmetic_eval(c1, c2, '+');
    assert!(!result.is_valid);
}

#[test]
fn test_arithmetic_eval_string_invalid_op() {
    let c1 = Cell::new_string("Hello".into());
    let c2 = Cell::new_string("World".into());
    let result = arithmetic_eval(c1, c2, '*');
    assert!(!result.is_valid);
}

#[test]
fn test_return_optype_mixed_cases() {
    assert_eq!(return_optype('+'), 1);
    assert_eq!(return_optype('-'), 2);
    assert_eq!(return_optype('*'), 3);
    assert_eq!(return_optype('/'), 4);
    assert_eq!(return_optype('x'), -1);
    assert_eq!(return_optype('='), -1);
}

#[test]
fn test_arithmetic_eval_float_minus_int() {
    let c1 = Cell::new_float(10.5);
    let c2 = Cell::new_int(2);
    let result = arithmetic_eval(c1, c2, '-');
    assert_eq!(result.value, CellValue::Float(8.5));
    assert!(result.is_valid);
}

#[test]
fn test_arithmetic_eval_string_concat() {
    let c1 = Cell::new_string("Hi ".to_string());
    let c2 = Cell::new_string("there".to_string());
    let result = arithmetic_eval(c1, c2, '+');
    assert_eq!(result.value, CellValue::String("Hi there".to_string()));
    assert!(result.is_valid);
}

#[test]
fn test_arithmetic_eval_int_float_div() {
    let c1 = Cell::new_int(7);
    let c2 = Cell::new_float(2.0);
    let result = arithmetic_eval(c1, c2, '/');
    assert_eq!(result.value, CellValue::Float(3.5));
    assert!(result.is_valid);
}

#[test]
fn test_arithmetic_eval_float_float_mul() {
    let c1 = Cell::new_float(2.5);
    let c2 = Cell::new_float(2.0);
    let result = arithmetic_eval(c1, c2, '*');
    assert_eq!(result.value, CellValue::Float(5.0));
    assert!(result.is_valid);
}

#[test]
fn test_arithmetic_eval_string_subtract_should_fail() {
    let c1 = Cell::new_string("Test".into());
    let c2 = Cell::new_string("Case".into());
    let result = arithmetic_eval(c1, c2, '-');
    assert!(!result.is_valid);
}

#[test]
fn test_arithmetic_eval_mixed_float_int_div_zero() {
    let c1 = Cell::new_int(10);
    let c2 = Cell::new_float(0.0);
    let result = arithmetic_eval(c1, c2, '/');
    assert!(!result.is_valid);
}

#[test]
fn test_arithmetic_eval_float_int_invalid_op() {
    let c1 = Cell::new_float(2.5);
    let c2 = Cell::new_int(3);
    let result = arithmetic_eval(c1, c2, '^');
    assert!(!result.is_valid);
}
#[test]
fn test_arithmetic_eval_int_mul() {
    let c1 = Cell::new_int(3);
    let c2 = Cell::new_int(4);
    let result = arithmetic_eval(c1, c2, '*');
    assert_eq!(result.value, CellValue::Int(12));
    assert!(result.is_valid);
}

#[test]
fn test_arithmetic_eval_int_invalid_op() {
    let c1 = Cell::new_int(3);
    let c2 = Cell::new_int(4);
    let result = arithmetic_eval(c1, c2, '^');
    assert!(!result.is_valid);
}

// #[test]
// fn test_arithmetic_eval_float_add_float() {
//     let c1 = Cell::new_float(1.1);
//     let c2 = Cell::new_float(2.2);
//     let result = arithmetic_eval(c1, c2, '+');
//     assert_eq!(result.value, CellValue::Float(3.3));
//     assert!(result.is_valid);
// }

#[test]
fn test_arithmetic_eval_float_invalid_op() {
    let c1 = Cell::new_float(1.0);
    let c2 = Cell::new_float(2.0);
    let result = arithmetic_eval(c1, c2, '%');
    assert!(!result.is_valid);
}

#[test]
fn test_arithmetic_eval_float_mul_int() {
    let c1 = Cell::new_float(1.5);
    let c2 = Cell::new_int(2);
    let result = arithmetic_eval(c1, c2, '*');
    assert_eq!(result.value, CellValue::Float(3.0));
    assert!(result.is_valid);
}

#[test]
fn test_arithmetic_eval_int_mul_float() {
    let c1 = Cell::new_int(2);
    let c2 = Cell::new_float(1.5);
    let result = arithmetic_eval(c1, c2, '*');
    assert_eq!(result.value, CellValue::Float(3.0));
    assert!(result.is_valid);
}

#[test]
fn test_arithmetic_eval_string_unknown_op() {
    let c1 = Cell::new_string("foo".to_string());
    let c2 = Cell::new_string("bar".to_string());
    let result = arithmetic_eval(c1, c2, '%');
    assert!(!result.is_valid);
}

#[test]
fn test_arithmetic_eval_int_string() {
    let c1 = Cell::new_int(42);
    let c2 = Cell::new_string("oops".into());
    let result = arithmetic_eval(c1, c2, '+');
    assert!(!result.is_valid);
}

#[test]
fn test_arithmetic_eval_float_string() {
    let c1 = Cell::new_float(3.14);
    let c2 = Cell::new_string("bad".into());
    let result = arithmetic_eval(c1, c2, '*');
    assert!(!result.is_valid);
}

#[test]
fn test_arithmetic_eval_debug_log_on_invalid_combo() {
    let c1 = Cell::new_string("bad".into());
    let c2 = Cell::new_int(1);
    let result = arithmetic_eval(c1, c2, '+');
    assert!(!result.is_valid);
}
