use crate::graph::{Graph, Formula};
use crate::function::*;
use std::i32;

pub static mut HAS_CYCLE: bool = false;
pub static mut INVALID_RANGE: bool = false;

pub fn cell_parser(a: &str, cols: usize, rows: usize, start: usize, end: usize, _graph: &Graph) -> Option<usize> {
    // let slice = &a[start..=end];
    if end >= a.len() || start > end {
        return None;
    }
    let slice = &a[start..=end];
    
    let mut col = 0;
    let mut row = 0;
    let mut digit_found = false;

    for ch in slice.chars() {
        if ch.is_ascii_alphabetic() {
            if digit_found {
                return None;
            }
            col = col * 26 + (ch.to_ascii_uppercase() as usize - 'A' as usize + 1);
        } else if ch.is_ascii_digit() {
            row = row * 10 + (ch as usize - '0' as usize);
            digit_found = true;
        } else {
            return None;
        }
    }

    if col == 0 || row == 0 {
        return None;
    }

    let col_idx = col - 1;
    let row_idx = row - 1;

    if col_idx >= cols || row_idx >= rows {
        return None;
    }

    Some(row_idx * cols + col_idx)
}

fn is_arithmetic(expr: &str) -> bool {
    expr.contains('+') || expr.contains('-') || expr.contains('*') || expr.contains('/')
}

pub fn parser(input: &str, cols: usize, rows: usize, arr: &mut [i32], graph: &mut Graph, formula_array: &mut [Formula]) -> i32 {
    unsafe {
        HAS_CYCLE = false;
        INVALID_RANGE = false;
    }
    
    let trimmed = input.trim();
    
    let eq_pos = trimmed.find('=');
    if eq_pos.is_none() {
        return -1;
    }

    let eq_index = eq_pos.unwrap();
    let left = &trimmed[..eq_index];
    // let right = &trimmed[eq_index + 1..];
    let right = &trimmed[eq_index + 1..].trim();

    if eq_index == 0 || eq_index > trimmed.len() {
        return -1;
    }
    
    let left_cell = match cell_parser(trimmed, cols, rows, 0, eq_index - 1, graph) {
        Some(idx) => idx,
        None => return -1,
    };

    // Save old state for rollback if needed
    let old_value = arr[left_cell];
    let old_formula = formula_array[left_cell];

    // Clear old edges if any
    if old_formula.op_type != 0 {
        graph.delete_edge(left_cell, old_formula.op_info1 as usize);
        if (1..=8).contains(&old_formula.op_type) {
            graph.delete_edge(left_cell, old_formula.op_info2 as usize);
        }
        if (9..=13).contains(&old_formula.op_type) {
            graph.delete_range(left_cell);
        }
    }

    let mut status = -1;

    if right.starts_with("MIN(") {
        status = min_func(trimmed, cols, rows, eq_index, trimmed.len(), arr, graph, formula_array);
    } else if right.starts_with("MAX(") {
        status = max_func(trimmed, cols, rows, eq_index, trimmed.len(), arr, graph, formula_array);
    } else if right.starts_with("AVG(") {
        status = avg_func(trimmed, cols, rows, eq_index, trimmed.len(), arr, graph, formula_array);
    } else if right.starts_with("SUM(") {
        status = sum_func(trimmed, cols, rows, eq_index, trimmed.len(), arr, graph, formula_array);
    } else if right.starts_with("STDEV(") {
        status = stdev_func(trimmed, cols, rows, eq_index, trimmed.len(), arr, graph, formula_array);
    } else if right.starts_with("SLEEP(") {
        // Ensure we have a valid SLEEP() format
        if right.ends_with(")") {
            let value_str = &right[6..right.len() - 1].trim(); // Extract value inside SLEEP()
            if let Ok(sleep_value) = value_str.parse::<i32>() {
                status = sleep_func(trimmed, cols, rows, eq_index, trimmed.len(), arr, graph, formula_array);
                // SLEEP value is parsed as expected
            } else if let Some(idx) = cell_parser(value_str, cols, rows, 0, value_str.len() - 1, graph) {
                graph.add_edge(idx, left_cell);
                formula_array[left_cell] = Formula {
                    op_type: 14, // Assuming 14 represents SLEEP with cell reference
                    op_info1: idx as i32,
                    op_info2: 0,
                };
                status = sleep_func(trimmed, cols, rows, eq_index, trimmed.len(), arr, graph, formula_array);
            } else {
                // Invalid number or cell reference inside SLEEP()
                return -1;
            }
        } else {
            // Invalid SLEEP syntax, missing closing parenthesis
            return -1;
        }
    }
    
     else if right.starts_with('-') && right[1..].chars().all(|c| c.is_ascii_digit()) {
        let const_val = right.parse::<i32>().unwrap_or(i32::MIN);
        // println!("const_val: {}", const_val);
        formula_array[left_cell] = Formula {
            op_type: 0,
            op_info1: const_val,
            op_info2: 0,
        };

        arr[left_cell] = const_val;
        
        // println!("************8{}", arr[left_cell]);
        status = 1;
        // return 1;
    } else if is_arithmetic(right) {
        // println!("Arithmetic expression detected: {}", right);
        // Parse left and right of operator
        let ops = ['+', '-', '*', '/'];
        let (op_index, op_char) = right
            .char_indices()
            .find(|(_, ch)| ops.contains(ch))
            .unwrap_or((0, '+'));

        let left_expr = right[..op_index].trim();
        let right_expr = right[op_index + 1..].trim();

        let is_left_cell = cell_parser(left_expr, cols, rows, 0, left_expr.len() - 1, graph).is_some();
        let is_right_cell = cell_parser(right_expr, cols, rows, 0, right_expr.len() - 1, graph).is_some();

        let (left_val, right_val, op_type) = if is_left_cell && is_right_cell {
            let idx1 = cell_parser(left_expr, cols, rows, 0, left_expr.len() - 1, graph).unwrap();
            let idx2 = cell_parser(right_expr, cols, rows, 0, right_expr.len() - 1, graph).unwrap();
            graph.add_edge(idx1, left_cell);
            graph.add_edge(idx2, left_cell);
            formula_array[left_cell] = Formula {
                op_type: match op_char {
                    '+' => 5,
                    '-' => 6,
                    '*' => 7,
                    '/' => 8,
                    _ => 0,
                },
                op_info1: idx1 as i32,
                op_info2: idx2 as i32,
            };
            (arr[idx1], arr[idx2], formula_array[left_cell].op_type)
        } else if is_left_cell {
            let idx1 = cell_parser(left_expr, cols, rows, 0, left_expr.len() - 1, graph).unwrap();
            let val2 = right_expr.parse::<i32>().unwrap_or(i32::MIN);
            graph.add_edge(idx1, left_cell);
            formula_array[left_cell] = Formula {
                op_type: match op_char {
                    '+' => 1,
                    '-' => 2,
                    '*' => 3,
                    '/' => 4,
                    _ => 0,
                },
                op_info1: idx1 as i32,
                op_info2: val2,
            };
            (arr[idx1], val2, formula_array[left_cell].op_type)
        } else if is_right_cell {
            let idx2 = cell_parser(right_expr, cols, rows, 0, right_expr.len() - 1, graph).unwrap();
            let val1 = left_expr.parse::<i32>().unwrap_or(i32::MIN);
            graph.add_edge(idx2, left_cell);
            formula_array[left_cell] = Formula {
                op_type: 15,
                op_info1: val1,
                op_info2: idx2 as i32,
            };
            (val1, arr[idx2], 15)
        } else {
            let val1 = left_expr.parse::<i32>().unwrap_or(i32::MIN);
            let val2 = right_expr.parse::<i32>().unwrap_or(i32::MIN);
            formula_array[left_cell] = Formula {
                op_type: 0,
                op_info1: match op_char {
                    '+' => val1 + val2,
                    '-' => val1 - val2,
                    '*' => val1 * val2,
                    '/' => if val2 != 0 { val1 / val2 } else { i32::MIN },
                    _ => i32::MIN,
                },
                op_info2: 0,
            };
            (val1, val2, 0)
        };

        arr[left_cell] = if op_type == 15 {
            if right_val == 0 {
                i32::MIN
            } else {
                left_val / right_val
            }
        } else if right_val == i32::MIN || left_val == i32::MIN {
            i32::MIN
        } else {
            match op_char {
                '+' => left_val + right_val,
                '-' => left_val - right_val,
                '*' => left_val * right_val,
                '/' => if right_val != 0 { left_val / right_val } else { i32::MIN },
                _ => i32::MIN,
            }
        };

        status = 1;
    } else {
        // Plain value or reference
        if right.starts_with('-') && right[1..].chars().all(|c| c.is_ascii_digit()) {
            let const_val = right.parse::<i32>().unwrap_or(i32::MIN);
            // println!("const_val: {}", const_val);
            formula_array[left_cell] = Formula {
                op_type: 0,
                op_info1: const_val,
                op_info2: 0,
            };
            arr[left_cell] = const_val;
            return 1;
        }
        
        if let Ok(const_val) = right.parse::<i32>() {
            formula_array[left_cell] = Formula {
                op_type: 0,
                op_info1: const_val,
                op_info2: 0,
            };
            arr[left_cell] = const_val;
            status = 1;
        } else if let Some(idx) = cell_parser(right, cols, rows, 0, right.len() - 1, graph) {
            graph.add_edge(idx, left_cell);
            formula_array[left_cell] = Formula {
                op_type: 1,
                op_info1: idx as i32,
                op_info2: 0,
            };
            arr[left_cell] = arr[idx];
            status = 1;
        }
    }
    
    
    if status == 1 {
        // println!("{}", arr[left_cell]);
        unsafe {
            HAS_CYCLE = false;
            
        }
        // status = 0;

        graph.recalc(cols, arr, left_cell, formula_array, unsafe { &mut HAS_CYCLE });
        // println!("Graph recalculation complete");
        // println!("{}", arr[left_cell]);
        unsafe {
            if HAS_CYCLE {
                // println!("Cycle detected, reverting to old value");
        
                // Step 1: Remove the edges added by the bad formula
                let new_formula = formula_array[left_cell];
                if (1..=8).contains(&new_formula.op_type) {
                    graph.delete_edge(new_formula.op_info1 as usize, left_cell);
                    graph.delete_edge(new_formula.op_info2 as usize, left_cell);
                } else if (9..=13).contains(&new_formula.op_type) {
                    graph.delete_range(left_cell);
                } else if [1, 15].contains(&new_formula.op_type) {
                    graph.delete_edge(new_formula.op_info1 as usize, left_cell);
                }
        
                // Step 2: Restore the old formula and value
                arr[left_cell] = old_value;
                formula_array[left_cell] = old_formula;
        
                // Step 3: Re-establish old edges
                if (1..=8).contains(&old_formula.op_type) {
                    graph.add_edge(old_formula.op_info1 as usize, left_cell);
                    graph.add_edge(old_formula.op_info2 as usize, left_cell);
                } else if (9..=13).contains(&old_formula.op_type) {
                    graph.add_range_to_graph(old_formula.op_info1 as usize, old_formula.op_info2 as usize, left_cell);
                } else if [1, 15].contains(&old_formula.op_type) {
                    graph.add_edge(old_formula.op_info1 as usize, left_cell);
                }
        
                return -1;
            }
        }
    }

    status
}
