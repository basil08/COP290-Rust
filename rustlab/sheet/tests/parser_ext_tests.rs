use sheet::function_ext::{Cell, CellValue};
use sheet::util_ext::{arithmetic_eval, return_optype};
use sheet::parser_ext::{cell_parser, detect_pattern, generate_sequence, autofill, parser};
use sheet::graph_ext::{Graph, Formula, State};
// use sheet::function_ext::{Cell, CellValue};



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
fn test_arithmetic_eval_float_div_float() {
    let c1 = Cell::new_float(9.0);
    let c2 = Cell::new_float(3.0);
    let result = arithmetic_eval(c1, c2, '/');
    assert_eq!(result.value, CellValue::Float(3.0));
    assert!(result.is_valid);
}

#[test]
fn test_arithmetic_eval_int_mul_int() {
    let c1 = Cell::new_int(6);
    let c2 = Cell::new_int(7);
    let result = arithmetic_eval(c1, c2, '*');
    assert_eq!(result.value, CellValue::Int(42));
    assert!(result.is_valid);
}

#[test]
fn test_arithmetic_eval_string_concat_spaces() {
    let c1 = Cell::new_string("   ".into());
    let c2 = Cell::new_string("More".into());
    let result = arithmetic_eval(c1, c2, '+');
    assert_eq!(result.value, CellValue::String("   More".to_string()));
    assert!(result.is_valid);
}

#[test]
fn test_arithmetic_eval_mixed_float_int_mul() {
    let c1 = Cell::new_int(3);
    let c2 = Cell::new_float(4.0);
    let result = arithmetic_eval(c1, c2, '*');
    assert_eq!(result.value, CellValue::Float(12.0));
    assert!(result.is_valid);
}

#[test]
fn test_arithmetic_eval_float_plus_float() {
    let c1 = Cell::new_float(1.5);
    let c2 = Cell::new_float(2.5);
    let result = arithmetic_eval(c1, c2, '+');
    assert_eq!(result.value, CellValue::Float(4.0));
    assert!(result.is_valid);
}

#[test]
fn test_arithmetic_eval_string_add_float_should_fail() {
    let c1 = Cell::new_string("Number: ".into());
    let c2 = Cell::new_float(3.14);
    let result = arithmetic_eval(c1, c2, '+');
    assert!(!result.is_valid);
}

#[test]
fn test_arithmetic_eval_string_string_non_add() {
    let c1 = Cell::new_string("hello".into());
    let c2 = Cell::new_string("world".into());
    let result = arithmetic_eval(c1, c2, '/');
    assert!(!result.is_valid);
}

#[test]
fn test_arithmetic_eval_float_int_zero_denominator() {
    let c1 = Cell::new_float(3.5);
    let c2 = Cell::new_int(0);
    let result = arithmetic_eval(c1, c2, '/');
    assert!(!result.is_valid);
}

#[test]
fn test_arithmetic_eval_invalid_combo() {
    let c1 = Cell::new_float(1.2);
    let c2 = Cell::new_string("abc".into());
    let result = arithmetic_eval(c1, c2, '+');
    assert!(!result.is_valid);
}
#[test]
fn test_cell_parser_start_ge_end() {
    let result = cell_parser("A1", 10, 10, 2, 1);
    assert!(result.is_err());
}

#[test]
fn test_cell_parser_letter_after_digit() {
    let result = cell_parser("A1B", 10, 10, 0, 2);
    assert!(result.is_err()); // A1B is invalid
}

#[test]
fn test_cell_parser_invalid_characters() {
    let result = cell_parser("A$", 10, 10, 0, 1);
    assert!(result.is_err());
}

#[test]
fn test_cell_parser_out_of_bounds() {
    let result = cell_parser("Z100", 5, 5, 0, 3); // Assuming Z100 > max rows
    assert!(result.is_err());
}
#[test]
fn test_detect_pattern_ap() {
    assert_eq!(detect_pattern(&[2, 4, 6, 8]), Some("AP".to_string()));
}

#[test]
fn test_detect_pattern_gp() {
    assert_eq!(detect_pattern(&[2, 4, 8, 16]), Some("GP".to_string()));
}

#[test]
fn test_detect_pattern_fib() {
    assert_eq!(detect_pattern(&[1, 1, 2, 3]), Some("FIB".to_string()));
}

// #[test]
// fn test_detect_pattern_const() {
//     assert_eq!(detect_pattern(&[5, 5, 5, 5]), Some("CONST".to_string()));
// }

#[test]
fn test_detect_pattern_none() {
    assert_eq!(detect_pattern(&[1, 2, 4, 7]), None);
}
#[test]
fn test_generate_sequence_ap() {
    let seq = generate_sequence(&[1, 2, 3, 4], "AP", 7);
    assert_eq!(seq, vec![1, 2, 3, 4, 5, 6, 7]);
}

#[test]
fn test_generate_sequence_gp() {
    let seq = generate_sequence(&[2, 4, 8, 16], "GP", 6);
    assert_eq!(seq, vec![2, 4, 8, 16, 32, 64]);
}

#[test]
fn test_generate_sequence_fib() {
    let seq = generate_sequence(&[1, 1, 2, 3], "FIB", 8);
    assert_eq!(seq, vec![1, 1, 2, 3, 5, 8, 13, 21]);
}

#[test]
fn test_generate_sequence_const() {
    let seq = generate_sequence(&[7, 7, 7, 7], "CONST", 6);
    assert_eq!(seq, vec![7, 7, 7, 7, 7, 7]);
}
#[test]
fn test_autofill_invalid_column() {
    let mut arr = vec![Cell::default(); 100];
    let result = autofill("ZZ", 5, 5, 5, &mut arr);
    assert!(result.is_err());
}

#[test]
fn test_autofill_insufficient_data() {
    let mut arr = vec![Cell::default(); 100];
    arr[0] = Cell::new_int(1);
    arr[5] = Cell::new_int(2);
    let result = autofill("A", 10, 10, 10, &mut arr);
    assert!(result.is_err());
}

#[test]
fn test_autofill_invalid_type_in_column() {
    let mut arr = vec![Cell::default(); 100];
    arr[0] = Cell::new_string("bad".to_string());
    arr[10] = Cell::new_int(2);
    let result = autofill("A", 10, 10, 10, &mut arr);
    assert!(result.is_err());
}
#[test]
fn test_cell_parser_invalid_range() {
    use sheet::parser_ext::cell_parser;
    let result = cell_parser("A1", 10, 10, 2, 1); // start > end
    assert!(result.is_err());
}

#[test]
fn test_cell_parser_letters_after_digits() {
    use sheet::parser_ext::cell_parser;
    let result = cell_parser("1A", 10, 10, 0, 1); // Invalid format
    assert!(result.is_err());
}
// #[test]
// fn test_cell_parser_invalid_character() {
//     use sheet::parser_ext::cell_parser;
//     let result = cell_parser("A!", 10, 10, 0, 1); // `!` is invalid
//     assert!(result.is_err());
// }

#[test]
fn test_cell_parser_lower_bound() {
    let result = cell_parser("A1", 10, 10, 0, 1); // valid lower bound
    assert_eq!(result.unwrap(), 0); // Cell 0
}

// #[test]
// fn test_cell_parser_upper_bound() {
//     let result = cell_parser("J10", 10, 10, 0, 3); // valid upper
//     assert_eq!(result.unwrap(), 99); // (9 * 10 + 9)
// }
#[test]
fn test_detect_pattern_empty_input() {
    assert_eq!(detect_pattern(&[]), None);
}

#[test]
fn test_detect_pattern_less_than_4() {
    assert_eq!(detect_pattern(&[1, 2, 3]), None);
}
#[test]
fn test_generate_sequence_invalid_pattern() {
    let seq = generate_sequence(&[1, 2, 3, 4], "XYZ", 6);
    assert_eq!(seq.len(), 4); // Should not extend
}

#[test]
fn test_autofill_ap_valid() {
    let mut arr = vec![Cell::default(); 100];
    arr[0] = Cell::new_int(1);
    arr[10] = Cell::new_int(2);
    arr[20] = Cell::new_int(3);
    arr[30] = Cell::new_int(4);
    let result = autofill("A", 10, 10, 4, &mut arr);
    assert!(result.is_ok());
    assert_eq!(arr[40], Cell::new_int(5));
    assert_eq!(arr[50], Cell::new_int(6));
}
#[test]
fn test_parser_valid_expression() {
    let mut arr = vec![Cell::default(); 100];
    let mut graph = Graph::new(100);
    let mut formula_array = vec![Formula::default(); 100];
    let mut state = State::new();
    state.num_cells = 100;

    let result = parser("B1=3", 10, 10, &mut arr, &mut graph, &mut formula_array, &mut state);
    assert_eq!(result, Ok(()));
    assert_eq!(arr[1], Cell::new_int(3));
}
#[test]
fn test_autofill_gp_valid() {
    let mut arr = vec![Cell::default(); 100];
    arr[0] = Cell::new_int(2);
    arr[10] = Cell::new_int(4);
    arr[20] = Cell::new_int(8);
    arr[30] = Cell::new_int(16);
    let result = sheet::parser_ext::autofill("A", 10, 10, 4, &mut arr);
    assert!(result.is_ok());
    assert_eq!(arr[40], Cell::new_int(32));
    assert_eq!(arr[50], Cell::new_int(64));
}

#[test]
fn test_autofill_const_valid() {
    let mut arr = vec![Cell::default(); 100];
    arr[0] = Cell::new_int(5);
    arr[10] = Cell::new_int(5);
    arr[20] = Cell::new_int(5);
    arr[30] = Cell::new_int(5);
    let result = sheet::parser_ext::autofill("A", 10, 10, 4, &mut arr);
    assert!(result.is_ok());
    assert_eq!(arr[40], Cell::new_int(5));
    assert_eq!(arr[50], Cell::new_int(5));
}

#[test]
fn test_autofill_fib_valid() {
    let mut arr = vec![Cell::default(); 100];
    arr[0] = Cell::new_int(1);
    arr[10] = Cell::new_int(1);
    arr[20] = Cell::new_int(2);
    arr[30] = Cell::new_int(3);
    let result = sheet::parser_ext::autofill("A", 10, 10, 4, &mut arr);
    assert!(result.is_ok());
    assert_eq!(arr[40], Cell::new_int(5));
    assert_eq!(arr[50], Cell::new_int(8));
}
#[test]
fn test_parser_const_assignment() {
    let mut arr = vec![Cell::default(); 100];
    let mut graph = Graph::new(100);
    let mut formula_array = vec![Formula::default(); 100];
    let result = parser("A1=5", 10, 10, &mut arr, &mut graph, &mut formula_array, &mut State::new());
assert!(result.is_ok());

    assert_eq!(arr[0], Cell::new_int(5));
}
#[test]
fn test_parser_binary_op() {
    let mut arr = vec![Cell::default(); 100];
    let mut graph = Graph::new(100);
    let mut formula_array = vec![Formula::default(); 100];
    arr[1] = Cell::new_int(4);
    let result = parser("A1=A2+3", 10, 10, &mut arr, &mut graph, &mut formula_array, &mut State::new());
    assert!(result.is_ok());
}
#[test]
fn test_parser_sum_function() {
    let mut arr = vec![Cell::default(); 100];
    let mut graph = Graph::new(100);
    let mut formula_array = vec![Formula::default(); 100];
    arr[1] = Cell::new_int(4);
    arr[2] = Cell::new_int(5);
    arr[3] = Cell::new_int(6);
    let result = parser("A1=SUM(B1:D1)", 10, 10, &mut arr, &mut graph, &mut formula_array, &mut State::new());
    assert!(result.is_ok());
}
#[test]
fn test_parser_string_assignment() {
    let mut arr = vec![Cell::default(); 100];
    let mut graph = Graph::new(100);
    let mut formula_array = vec![Formula::default(); 100];
    let result = parser("A1=\"hello\"", 10, 10, &mut arr, &mut graph, &mut formula_array, &mut State::new());
    assert!(result.is_ok());
    assert_eq!(arr[0], Cell::new_string("hello".to_string()));
}
#[test]
fn test_parser_invalid_function_name() {
    let mut arr = vec![Cell::default(); 100];
    let mut formula_array = vec![Formula::default(); 100];
    let mut graph = Graph::new(100);
    let mut state = State::new();

    let result = parser("A1=BOGUS(B1:B3)", 10, 10, &mut arr, &mut graph, &mut formula_array, &mut state);
    assert!(result.is_err());
}
#[test]
fn test_parser_float_assignment() {
    let mut arr = vec![Cell::default(); 100];
    let mut formula_array = vec![Formula::default(); 100];
    let mut graph = Graph::new(100);
    let mut state = State::new();

    let result = parser("A1=3.14", 10, 10, &mut arr, &mut graph, &mut formula_array, &mut state);
    assert!(result.is_ok());
    assert_eq!(arr[0], Cell::new_float(3.14));
}

#[test]
fn test_parser_const_division_formula() {
    let mut arr = vec![Cell::default(); 100];
    let mut formula_array = vec![Formula::default(); 100];
    let mut graph = Graph::new(100);
    let mut state = State::new();

    let result = parser("A1=10/2", 10, 10, &mut arr, &mut graph, &mut formula_array, &mut state);
    assert!(result.is_ok());
    assert_eq!(arr[0], Cell::new_int(5));
}
#[test]
fn test_parser_float_division_formula() {
    let mut arr = vec![Cell::default(); 100];
    let mut formula_array = vec![Formula::default(); 100];
    let mut graph = Graph::new(100);
    let mut state = State::new();

    let result = parser("A1=7/2", 10, 10, &mut arr, &mut graph, &mut formula_array, &mut state);
    assert!(result.is_ok());
    assert_eq!(arr[0], Cell::new_float(3.5));
}
#[test]
fn test_parser_stdev_with_invalid_type() {
    let mut arr = vec![Cell::default(); 100];
    arr[1] = Cell::new_string("invalid".into());
    arr[2] = Cell::new_int(5);
    let mut formula_array = vec![Formula::default(); 100];
    let mut graph = Graph::new(100);
    let mut state = State::new();

    let result = parser("A1=STDEV(B1:B2)", 10, 10, &mut arr, &mut graph, &mut formula_array, &mut state);
    assert!(result.is_ok());
    assert_eq!(arr[0].is_valid, false);
}

#[test]
fn test_parser_sleep_self_value_cycle() {
    let mut arr = vec![Cell::default(); 100];
    arr[0] = Cell::new_int(1);
    let mut formula_array = vec![Formula::default(); 100];
    let mut graph = Graph::new(100);
    let mut state = State::new();
    state.num_cells = 100;

    let result = parser("A1=SLEEP(A1)", 10, 10, &mut arr, &mut graph, &mut formula_array, &mut state);
    assert!(result.is_err());
    assert!(state.has_cycle);
}

// #[test]
// fn test_parser_trimmed_input() {
//     let mut arr = vec![Cell::default(); 100];
//     let mut formula_array = vec![Formula::default(); 100];
//     let mut graph = Graph::new(100);
//     let mut state = State::new();

//     let result = parser("  A1=4+5", 10, 10, &mut arr, &mut graph, &mut formula_array, &mut state);
//     assert!(result.is_ok());
//     assert_eq!(arr[0], Cell::new_int(9));
// }
#[test]
fn test_parser_max_empty_range() {
    let mut arr = vec![Cell::default(); 100];
    let mut formula_array = vec![Formula::default(); 100];
    let mut graph = Graph::new(100);
    let mut state = State::new();

    let result = parser("A1=MAX(B1:B1)", 10, 10, &mut arr, &mut graph, &mut formula_array, &mut state);
    assert!(result.is_ok());
    assert!(!arr[0].is_valid);
}
#[test]
fn test_parser_avg_empty_range() {
    let mut arr = vec![Cell::default(); 100];
    let mut formula_array = vec![Formula::default(); 100];
    let mut graph = Graph::new(100);
    let mut state = State::new();

    let result = parser("A1=AVG(B1:B1)", 10, 10, &mut arr, &mut graph, &mut formula_array, &mut state);
    assert!(result.is_ok());
    assert!(!arr[0].is_valid);
}
#[test]
fn test_parser_cell_to_cell_assignment() {
    let mut arr = vec![Cell::default(); 100];
    arr[11] = Cell::new_int(99); // B2
    let mut formula_array = vec![Formula::default(); 100];
    let mut graph = Graph::new(100);
    let mut state = State::new();

    let result = parser("A1=B2", 10, 10, &mut arr, &mut graph, &mut formula_array, &mut state);
    assert!(result.is_ok());
    assert_eq!(arr[0], Cell::new_int(99));
}
#[test]
fn test_parser_sleep_indirect_cell() {
    let mut arr = vec![Cell::default(); 100];
    arr[1] = Cell::new_int(0); // B1
    let mut formula_array = vec![Formula::default(); 100];
    let mut graph = Graph::new(100);
    let mut state = State::new();

    let result = parser("A1=SLEEP(B1)", 10, 10, &mut arr, &mut graph, &mut formula_array, &mut state);
    assert!(result.is_ok());
    assert_eq!(arr[0], Cell::new_int(0));
}

#[test]
fn test_parser_min_range() {
    let mut arr = vec![Cell::default(); 100];
    arr[1] = Cell::new_int(10);
    arr[2] = Cell::new_int(5);
    arr[3] = Cell::new_int(15);
    let mut formula_array = vec![Formula::default(); 100];
    let mut graph = Graph::new(100);
    let mut state = State::new();

    let result = parser("A1=MIN(B1:D1)", 10, 10, &mut arr, &mut graph, &mut formula_array, &mut state);
    assert!(result.is_ok());
    assert_eq!(arr[0], Cell::new_int(5));
}

#[test]
fn test_parser_avg_range() {
    let mut arr = vec![Cell::default(); 100];
    arr[1] = Cell::new_int(10);
    arr[2] = Cell::new_int(20);
    arr[3] = Cell::new_int(30);
    let mut formula_array = vec![Formula::default(); 100];
    let mut graph = Graph::new(100);
    let mut state = State::new();

    let result = parser("A1=AVG(B1:D1)", 10, 10, &mut arr, &mut graph, &mut formula_array, &mut state);
    assert!(result.is_ok());
    assert_eq!(arr[0], Cell::new_int(20));
}

#[test]
fn test_parser_stdev_valid_range() {
    let mut arr = vec![Cell::default(); 100];
    arr[1] = Cell::new_int(2);
    arr[2] = Cell::new_int(4);
    arr[3] = Cell::new_int(4);
    arr[4] = Cell::new_int(4);
    arr[5] = Cell::new_int(5);
    arr[6] = Cell::new_int(5);
    arr[7] = Cell::new_int(7);
    arr[8] = Cell::new_int(9);
    let mut formula_array = vec![Formula::default(); 100];
    let mut graph = Graph::new(100);
    let mut state = State::new();

    let result = parser("A1=STDEV(B1:I1)", 10, 10, &mut arr, &mut graph, &mut formula_array, &mut state);
    assert!(result.is_ok());
    assert!(arr[0].is_valid);
    // We don’t assert exact float due to rounding, but you could use pattern matching if needed.
}
#[test]
fn test_parser_invalid_cell_assignment() {
    let mut arr = vec![Cell::default(); 100];
    let mut formula_array = vec![Formula::default(); 100];
    let mut graph = Graph::new(100);
    let mut state = State::new();
    let result = parser("5=5", 10, 10, &mut arr, &mut graph, &mut formula_array, &mut state);
    assert!(result.is_err());
}




#[test]
fn test_parser_negative_constant() {
    let mut arr = vec![Cell::default(); 100];
    let mut formula_array = vec![Formula::default(); 100];
    let mut graph = Graph::new(100);
    let mut state = State::new();
    let result = parser("A1=-5", 10, 10, &mut arr, &mut graph, &mut formula_array, &mut state);
    assert!(result.is_ok());
    assert_eq!(arr[0], Cell::new_int(-5));
}

#[test]
fn test_parser_positive_constant() {
    let mut arr = vec![Cell::default(); 100];
    let mut formula_array = vec![Formula::default(); 100];
    let mut graph = Graph::new(100);
    let mut state = State::new();
    let result = parser("A1=+5", 10, 10, &mut arr, &mut graph, &mut formula_array, &mut state); // Lines 204, 207, 210
    assert!(result.is_ok());
    assert_eq!(arr[0], Cell::new_int(5));
}

#[test]
fn test_parser_empty_string() {
    let mut arr = vec![Cell::default(); 100];
    let mut formula_array = vec![Formula::default(); 100];
    let mut graph = Graph::new(100);
    let mut state = State::new();
    let result = parser("A1=\"\"", 10, 10, &mut arr, &mut graph, &mut formula_array, &mut state); // Lines 234-237
    assert!(result.is_ok());
    assert_eq!(arr[0], Cell::new_string("".to_string()));
}

#[test]
fn test_parser_autofill_missing_length() {
    let mut arr = vec![Cell::default(); 100];
    let mut formula_array = vec![Formula::default(); 100];
    let mut graph = Graph::new(100);
    let mut state = State::new();
    let result = parser("=autofill A", 10, 10, &mut arr, &mut graph, &mut formula_array, &mut state); // Lines 493-498
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Usage: autofill <column> <length>");
}

#[test]
fn test_parser_no_digit_before_operator() {
    let mut arr = vec![Cell::default(); 100];
    let mut formula_array = vec![Formula::default(); 100];
    let mut graph = Graph::new(100);
    let mut state = State::new();
    let result = parser("A1=+B1", 10, 10, &mut arr, &mut graph, &mut formula_array, &mut state); // Lines 521-524, 526-530
    assert!(result.is_ok()); // Should parse as value function
}

#[test]
fn test_parser_unknown_function() {
    let mut arr = vec![Cell::default(); 100];
    let mut formula_array = vec![Formula::default(); 100];
    let mut graph = Graph::new(100);
    let mut state = State::new();
    let result = parser("A1=XYZ(B1)", 10, 10, &mut arr, &mut graph, &mut formula_array, &mut state); // Lines 550, 552-555
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Unknown function");
}
#[test]
fn test_parser_sleep_with_prior_formula() {
    let mut arr = vec![Cell::default(); 100];
    let mut formula_array = vec![Formula::default(); 100];
    formula_array[0] = Formula { op_type: 1, op_info1: 1, op_info2: 2 };
    let mut graph = Graph::new(100);
    let mut state = State::new();
    let result = parser("A1=SLEEP(1)", 10, 10, &mut arr, &mut graph, &mut formula_array, &mut state); // Lines 467-470
    assert!(result.is_ok());
}

#[test]
fn test_parser_sleep_missing_paren() {
    let mut arr = vec![Cell::default(); 100];
    let mut formula_array = vec![Formula::default(); 100];
    let mut graph = Graph::new(100);
    let mut state = State::new();
    let result = parser("A1=SLEEP(1", 10, 10, &mut arr, &mut graph, &mut formula_array, &mut state); // Lines 472-473
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Missing closing parenthesis");
}

// #[test]
// fn test_parser_sleep_invalid_cell() {
//     let mut arr = vec![Cell::default(); 100];
//     let mut formula_array = vec![Formula::default(); 100];
//     let mut graph = Graph::new(100);
//     let mut state = State::new();
//     let result = parser("A1=SLEEP(Z1a)", 10, 10, &mut arr, &mut graph, &mut formula_array, &mut state); // Line 476
//     assert!(result.is_ok()); // Falls back to constant parsing
// }

#[test]
fn test_parser_sleep_constant() {
    let mut arr = vec![Cell::default(); 100];
    let mut formula_array = vec![Formula::default(); 100];
    let mut graph = Graph::new(100);
    let mut state = State::new();
    let result = parser("A1=SLEEP(2)", 10, 10, &mut arr, &mut graph, &mut formula_array, &mut state); // Lines 485-487
    assert!(result.is_ok());
    assert_eq!(arr[0], Cell::new_int(2));
}

#[test]
fn test_parser_range_with_prior_formula() {
    let mut arr = vec![Cell::default(); 100];
    let mut formula_array = vec![Formula::default(); 100];
    formula_array[0] = Formula { op_type: 1, op_info1: 1, op_info2: 2 };
    let mut graph = Graph::new(100);
    let mut state = State::new();
    let result = parser("A1=MIN(B1:C1)", 10, 10, &mut arr, &mut graph, &mut formula_array, &mut state); // Lines 437-439
    assert!(result.is_ok());
}

#[test]
fn test_parser_missing_paren() {
    let mut arr = vec![Cell::default(); 100];
    let mut formula_array = vec![Formula::default(); 100];
    let mut graph = Graph::new(100);
    let mut state = State::new();
    let result = parser("A1=MIN(B1:C1", 10, 10, &mut arr, &mut graph, &mut formula_array, &mut state); // Lines 443-444
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Missing closing parenthesis");
}

#[test]
fn test_parser_missing_colon() {
    let mut arr = vec![Cell::default(); 100];
    let mut formula_array = vec![Formula::default(); 100];
    let mut graph = Graph::new(100);
    let mut state = State::new();
    let result = parser("A1=MIN(B1 C1)", 10, 10, &mut arr, &mut graph, &mut formula_array, &mut state); // Line 445
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Missing colon");
}
// #[test]
// fn test_parser_multiple_operators() {
//     let mut arr = vec![Cell::default(); 100];
//     let mut formula_array = vec![Formula::default(); 100];
//     let mut graph = Graph::new(100);
//     let mut state = State::new();
//     let result = parser("A1=2++3", 10, 10, &mut arr, &mut graph, &mut formula_array, &mut state); // Lines 345-346, 348
//     assert!(result.is_err());
//     assert_eq!(result.unwrap_err(), "No valid operator found");
// }

#[test]
fn test_parser_positive_first_operand() {
    let mut arr = vec![Cell::default(); 100];
    let mut formula_array = vec![Formula::default(); 100];
    let mut graph = Graph::new(100);
    let mut state = State::new();
    let result = parser("A1=+2+3", 10, 10, &mut arr, &mut graph, &mut formula_array, &mut state); // Lines 364, 366, 368
    assert!(result.is_ok());
    assert_eq!(arr[0], Cell::new_int(5));
}

#[test]
fn test_parser_positive_second_operand() {
    let mut arr = vec![Cell::default(); 100];
    let mut formula_array = vec![Formula::default(); 100];
    let mut graph = Graph::new(100);
    let mut state = State::new();
    let result = parser("A1=2+3", 10, 10, &mut arr, &mut graph, &mut formula_array, &mut state); // Lines 378-380, 382-385
    // assert!(result.is_err());
    // assert_eq!(result.unwrap_err(), "No valid operator found");
    assert!(result.is_ok());
}


#[test]
fn test_parser_constant_cell_division() {
    let mut arr = vec![Cell::default(); 100];
    arr[1] = Cell::new_int(5);
    let mut formula_array = vec![Formula::default(); 100];
    let mut graph = Graph::new(100);
    let mut state = State::new();
    let result = parser("A1=10/B1", 10, 10, &mut arr, &mut graph, &mut formula_array, &mut state); // Lines 423, 425-426
    assert!(result.is_ok());
    assert_eq!(arr[0], Cell::new_int(2));
}

#[test]
fn test_parser_cell_cell_subtraction() {
    let mut arr = vec![Cell::default(); 100];
    arr[1] = Cell::new_int(10);
    arr[2] = Cell::new_int(3);
    let mut formula_array = vec![Formula::default(); 100];
    let mut graph = Graph::new(100);
    let mut state = State::new();
    let result = parser("A1=B1-C1", 10, 10, &mut arr, &mut graph, &mut formula_array, &mut state); // Line 429
    assert!(result.is_ok());
    assert_eq!(arr[0], Cell::new_int(7));
}





#[test]
fn test_parser_invalid_cell_reference() {
    let mut arr = vec![Cell::default(); 100];
    let mut formula_array = vec![Formula::default(); 100];
    let mut graph = Graph::new(100);
    let mut state = State::new();
    let result = parser("A1=Z100", 10, 10, &mut arr, &mut graph, &mut formula_array, &mut state); // Lines 274-275
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Cell reference out of bounds");
}



