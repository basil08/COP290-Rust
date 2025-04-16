use crate::graph::State;
use crate::graph::Graph;
use crate::graph::Formula;
use crate::function::Cell;


use crate::util::{arithmetic_eval, return_optype};
use crate::function::CellValue;

fn is_alpha(c: char) -> bool {
    c.is_ascii_uppercase() && ('A'..='Z').contains(&c)
}

fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

pub fn cell_parser(a: &str, c: i32, r: i32, start: usize, end: usize) -> Result<i32, &'static str> {
    let mut cell_col = 0;
    let mut cell_row = 0;
    let mut digit_found = false;

    for ch in a[start..=end].chars() {
        if is_alpha(ch) {
            if digit_found {
                return Err("Letters after digits not allowed");
            }
            cell_col = 26 * cell_col + (ch as u32 - 'A' as u32 + 1) as i32;
        } else if is_digit(ch) {
            cell_row = 10 * cell_row + (ch as u32 - '0' as u32) as i32;
            digit_found = true;
        } else {
            return Err("Invalid character in cell reference");
        }
    }

    let cell_col = cell_col - 1;
    let cell_row = cell_row - 1;

    if cell_col < 0 || cell_row < 0 || cell_col >= c || cell_row >= r {
        return Err("Cell reference out of bounds");
    }

    Ok(c * cell_row + cell_col)
}

fn value_func(
    
    a: &str,
    c: i32,
    r: i32,
    pos_equalto: usize,
    pos_end: usize,
    arr: &mut [Cell],
    graph: &mut Graph,
    formula_array: &mut [Formula],
    state: &mut State,
) -> Result<(), &'static str> {
    // println!("[DEBUG] Parsing value function: {}", &a[pos_equalto + 1..pos_end]);
    // println!("value function");
    let first_cell = cell_parser(a, c, r, 0, pos_equalto - 1)?;
    state.old_value = arr[first_cell as usize].clone();
    state.old_op_type = formula_array[first_cell as usize].op_type;
    state.old_op_info1 = formula_array[first_cell as usize].op_info1;
    state.old_op_info2 = formula_array[first_cell as usize].op_info2;

    if formula_array[first_cell as usize].op_type > 0 {
        graph.delete_edge(first_cell, c, formula_array);
    }

    let mut second_cell = -1;
    let mut is_cell = false;
    let mut is_negative = false;

    let mut pos = pos_equalto + 1;
    if pos < a.len() && (a[pos..].starts_with('-') || a[pos..].starts_with('+')) {
        is_negative = a[pos..].starts_with('-');
        pos += 1;
    }

    if a[pos..pos_end].starts_with('"') && a[pos..pos_end].ends_with('"') {
        // Handle string values
        let string_value = &a[pos + 1..pos_end - 1]; // Remove surrounding quotes
        arr[first_cell as usize] = Cell::new_string(string_value.to_string());
        // println!("[debug] {:?}", arr[first_cell as usize]);
        graph.add_formula(first_cell, 0, 0, 16, formula_array);
        graph.recalc(c, arr, first_cell, formula_array, state)?;
        // println!("Stored string in cell {}: {}", first_cell, string_value);
       
        return Ok(());
    } else if a[pos..pos_end].chars().all(is_digit) {
        second_cell = a[pos..pos_end].parse::<i32>().map_err(|_| "Invalid integer")?;
    } 
    else if a[pos..pos_end].chars().any(|ch| ch == '.') {
    let float_value = a[pos..pos_end].parse::<f64>().map_err(|_| "Invalid float")?;
    arr[first_cell as usize] = Cell::new_float(float_value);
    // println!("[DEBUG] Float value of cell : {:?}", arr[first_cell as usize]);
    // println!("[DEBUG] Float value: {}", float_value);
    graph.add_formula(first_cell, 0, 0, 17, formula_array);
    // println!("[DEBUG] Float value of cell : {:?}", arr[first_cell as usize]);
    
    graph.recalc(c, arr, first_cell, formula_array, state)?;
    // println!("[DEBUG] Float value of cell : {:?}", arr[first_cell as usize]);
    
    return Ok(());
    } 
    else {
        second_cell = cell_parser(a, c, r, pos, pos_end - 1)?;
        // println!("[DEBUG] Cell reference: {}", second_cell);
        is_cell = true;
        
    }

    // println!("[DEBUG] Before handling negative: second_cell = {}, is_negative = {}, is_cell = {}", second_cell, is_negative, is_cell);
    if is_negative && !is_cell {
        second_cell = -second_cell;
    }
    // println!("[DEBUG] After handling negative: second_cell = {}", second_cell);
    // println!("[DEBUG] Value of second_cell: {:?}", arr[second_cell as usize].value);
    if !is_cell && !a[pos..pos_end].starts_with('"') {
        arr[first_cell as usize] = Cell::new_int(second_cell);
        graph.add_formula(first_cell, second_cell, 0, 0, formula_array);
        graph.recalc(c, arr, first_cell, formula_array, state)?;
    } else if is_cell {
        let value = if is_negative {
            match arr[second_cell as usize].value {
                CellValue::Int(i) => Cell::new_int(-i),
                CellValue::Float(f) => Cell::new_float(-f),
                _ => Cell::invalid(),
            }
        } else {
            arr[second_cell as usize].clone()
        };
        // println!("[DEBUG] Value of second_cell: {:?}", value.value);
        arr[first_cell as usize] = value;
        // println!("[DEBUG] Value of first_cell: {:?}", arr[first_cell as usize].value);
        graph.add_edge(first_cell, second_cell as usize);
        // println!("[DEBUG] Value of first_cell: {:?}", arr[first_cell as usize].value);
       
        let op_type = -1;
        // let mut v;
        // if let CellValue::String(_) = arr[second_cell as usize].value {
        //     v = "".to_string();
        // } else if let CellValue::Float(_) = arr[second_cell as usize].value {
        //     v = 0.0;
        // } else {
        //     v = 0;
        // }
        
        graph.add_formula(first_cell, second_cell, 0, op_type, formula_array);
        // println!("[DEBUG] Value of first_cell: {:?}", arr[first_cell as usize].value);
       
        graph.recalc(c, arr, first_cell, formula_array, state)?;
        // println!("[DEBUG] Value of first_cell: {:?}", arr[first_cell as usize].value);
       
    }

    if state.has_cycle {
        arr[first_cell as usize] = state.old_value.clone();
        graph.delete_edge(first_cell, c, formula_array);
        formula_array[first_cell as usize] = Formula {
            op_type: state.old_op_type,
            op_info1: state.old_op_info1,
            op_info2: state.old_op_info2,
        };
        graph.add_edge_formula(first_cell, c, formula_array);
        return Err("Cycle detected");
    }

    Ok(())
}

fn arth_op(
    a: &str,
    c: i32,
    r: i32,
    pos_equalto: usize,
    pos_end: usize,
    arr: &mut [Cell],
    graph: &mut Graph,
    formula_array: &mut [Formula],
    state: &mut State
) -> Result<(), &'static str> {
    // println!("arithmetic operation");
    let mut operation = None;
    let mut opindex = None;
    // println!("[DEBUG] Parsing arithmetic operation: {}", &a[pos_equalto + 1..pos_end]);
    for (i, ch) in a[pos_equalto + 1..pos_end].chars().enumerate() {
        if "+-*/".contains(ch) && i + pos_equalto + 1 > pos_equalto + 1 {
            let prev = a[(i + pos_equalto)..].chars().next().unwrap();
            if !"+-*/".contains(prev) {
                operation = Some(ch);
                opindex = Some(i + pos_equalto + 1);
                break;
            }
        }
    }

    let (op, opindex) = operation.zip(opindex).ok_or("No valid operator found")?;

    let first_cell = cell_parser(a, c, r, 0, pos_equalto - 1)?;
    
    state.old_value = arr[first_cell as usize].clone();
    state.old_op_type = formula_array[first_cell as usize].op_type;
    state.old_op_info1 = formula_array[first_cell as usize].op_info1;
    state.old_op_info2 = formula_array[first_cell as usize].op_info2;

    if formula_array[first_cell as usize].op_type > 0 {
        graph.delete_edge(first_cell, c, formula_array);
    }

    let mut second_cell = 0;
    let mut third_cell = 0;
    let mut is1cell = false;
    let mut is2cell = false;
    let mut sign1 = 1;
    let mut sign2 = 1;

    let mut start = pos_equalto + 1;
    if a[start..].starts_with('-') {
        sign1 = -1;
        start += 1;
    } else if a[start..].starts_with('+') {
        start += 1;
    }

    let first_part = &a[start..opindex];
    if first_part.chars().any(is_alpha) {
        second_cell = cell_parser(a, c, r, start, opindex - 1)?;
        is1cell = true;
    } else {
        second_cell = first_part.parse::<i32>().map_err(|_| "Invalid first operand")? * sign1;
    }

    let mut second_start = opindex + 1;
    if a[second_start..].starts_with('-') {
        sign2 = -1;
        second_start += 1;
    } else if a[second_start..].starts_with('+') {
        second_start += 1;
    }

    let second_part = &a[second_start..pos_end];
    if second_part.chars().any(is_alpha) {
        third_cell = cell_parser(a, c, r, second_start, pos_end - 1)?;
        is2cell = true;
    } else {
        third_cell = second_part.parse::<i32>().map_err(|_| "Invalid second operand")? * sign2;
    }

    match (is1cell, is2cell) {
        (false, false) => {
            let res = arithmetic_eval(Cell::new_int(second_cell), Cell::new_int(third_cell), op);
            arr[first_cell as usize] = res.clone();
            if let CellValue::Int(value) = res.value {
                graph.add_formula(first_cell, value, 0, 0, formula_array);
            } else if let CellValue::Float(value) = res.value {
                graph.add_formula(first_cell, value as i32, 0, 17, formula_array);
            } else {
                return Err("Unsupported value type for arithmetic operation");
            }
            // println!("[DEBUG] Value of first_cell: {:?}", arr[first_cell as usize].value);
            // println!("operation: {:?} {} {:?}", arr[first_cell as usize].value, return_optype(op), arr[third_cell as usize].value);
        }
        (true, false) => {
            graph.add_edge(first_cell, second_cell as usize);
            graph.add_formula(first_cell, second_cell, third_cell, return_optype(op), formula_array);
        }
        (false, true) => {
            graph.add_edge(first_cell, third_cell as usize);
            let op_type = if op == '/' { 15 } else { return_optype(op) };
            graph.add_formula(first_cell, second_cell, third_cell, op_type, formula_array);
        }
        (true, true) => {
            graph.add_edge(first_cell, second_cell as usize);
            graph.add_edge(first_cell, third_cell as usize);
            graph.add_formula(first_cell, second_cell, third_cell, return_optype(op) + 4, formula_array);
        }
    };
    // println!("[DEBUG] Value of first_cell: {:?}", arr[first_cell as usize].value);

    graph.recalc(c, arr, first_cell, formula_array, state)?;
    // println!("[DEBUG] Value of first_cell: {:?}", arr[first_cell as usize].value);
    if state.has_cycle {
        arr[first_cell as usize] = state.old_value.clone();
        graph.delete_edge(first_cell, c, formula_array);
        formula_array[first_cell as usize] = Formula {
            op_type: state.old_op_type,
            op_info1: state.old_op_info1,
            op_info2: state.old_op_info2,
        };
        graph.add_edge_formula(first_cell, c, formula_array);
        return Err("Cycle detected");
    }

    Ok(())
}

fn range_func(
    a: &str,
    c: i32,
    r: i32,
    pos_equalto: usize,
    pos_end: usize,
    arr: &mut [Cell],
    graph: &mut Graph,
    formula_array: &mut [Formula],
    state: &mut State,
    op_type: i32,
) -> Result<(), &'static str> {
    let first_cell = cell_parser(a, c, r, 0, pos_equalto - 1)?;
    state.old_value = arr[first_cell as usize].clone();
    state.old_op_type = formula_array[first_cell as usize].op_type;
    state.old_op_info1 = formula_array[first_cell as usize].op_info1;
    state.old_op_info2 = formula_array[first_cell as usize].op_info2;

    if formula_array[first_cell as usize].op_type > 0 {
        graph.delete_edge(first_cell, c, formula_array);
    }

    let eq_str = &a[pos_equalto..];
    let open_paren = eq_str.find('(').map(|i| i + pos_equalto).ok_or("Missing opening parenthesis")?;
    let close_paren = eq_str.find(')').map(|i| i + pos_equalto).ok_or("Missing closing parenthesis")?;
    let colon_pos = a[open_paren + 1..].find(':').map(|i| i + open_paren + 1).ok_or("Missing colon")?;

    let range_start = cell_parser(a, c, r, open_paren + 1, colon_pos - 1)?;
    let range_end = cell_parser(a, c, r, colon_pos + 1, close_paren - 1)?;

    graph.add_formula(first_cell, range_start, range_end, op_type, formula_array);
    graph.add_range_to_graph(range_start, range_end, first_cell);
    graph.recalc(c, arr, first_cell, formula_array, state)?;

    if state.has_cycle {
        arr[first_cell as usize] = state.old_value.clone();
        graph.delete_edge(first_cell, c, formula_array);
        formula_array[first_cell as usize] = Formula {
            op_type: state.old_op_type,
            op_info1: state.old_op_info1,
            op_info2: state.old_op_info2,
        };
        graph.add_edge_formula(first_cell, c, formula_array);
        return Err("Cycle detected");
    }
    Ok(())
}

fn sleep_func(
    a: &str,
    c: i32,
    r: i32,
    pos_equalto: usize,
    pos_end: usize,
    arr: &mut [Cell],
    graph: &mut Graph,
    formula_array: &mut [Formula],
    state: &mut State
) -> Result<(), &'static str> {
    let target_cell = cell_parser(a, c, r, 0, pos_equalto - 1)?;
    state.old_value = arr[target_cell as usize].clone();
    state.old_op_type = formula_array[target_cell as usize].op_type;
    state.old_op_info1 = formula_array[target_cell as usize].op_info1;
    state.old_op_info2 = formula_array[target_cell as usize].op_info2;

    if formula_array[target_cell as usize].op_type > 0 {
        graph.delete_edge(target_cell, c, formula_array);
    }

    let eq_str = &a[pos_equalto..];
    let open_paren = eq_str.find('(').map(|i| i + pos_equalto).ok_or("Missing opening parenthesis")?;
    let close_paren = eq_str.find(')').map(|i| i + pos_equalto).ok_or("Missing closing parenthesis")?;

    let ref_cell = cell_parser(a, c, r, open_paren + 1, close_paren - 1);
    if let Ok(ref_cell) = ref_cell {
        graph.add_formula(target_cell, ref_cell, ref_cell, 14, formula_array);
        graph.add_edge(target_cell, ref_cell as usize);
    } else {
        let sleep_str = &a[open_paren + 1..close_paren];
        let sleep_value = sleep_str.parse::<i32>().map_err(|_| "Invalid sleep value")?;
        graph.add_formula(target_cell, target_cell, sleep_value, 14, formula_array);
    }

    graph.recalc(c, arr, target_cell, formula_array, state)?;

    if state.has_cycle {
        arr[target_cell as usize] = state.old_value.clone();
        graph.delete_edge(target_cell, c, formula_array);
        formula_array[target_cell as usize] = Formula {
            op_type: state.old_op_type,
            op_info1: state.old_op_info1,
            op_info2: state.old_op_info2,
        };
        graph.add_edge_formula(target_cell, c, formula_array);
        return Err("Cycle detected");
    }
    Ok(())
}

pub fn parser(
    a: &str,
    c: i32,
    r: i32,
    arr: &mut [Cell],
    graph: &mut Graph,
    formula_array: &mut [Formula],
    state: &mut State
) -> Result<(), &'static str> {
    state.num_cells = c as usize * r as usize;
    if a.starts_with('w') || a.starts_with('d') || a.starts_with('a') || a.starts_with('s') {
        return Ok(());
    }

    let pos_equalto = a.find('=').ok_or("No equals sign found")?;
    let pos_end = a.len();

    let mut value = false;
    let mut arth_exp = false;
    let mut func = false;
    let mut found_digit = false;

    for ch in a[pos_equalto + 1..pos_end].chars() {
        if ch == '(' {
            func = true;
            break;
        }
        if is_digit(ch) {
            found_digit = true;
        }
        if "+-*/".contains(ch) && found_digit {
            arth_exp = true;
            break;
        }
    }

    if !func && !arth_exp {
        value = true;
    }

    if func && arth_exp {
        return Err("Invalid input: Cannot mix function and arithmetic");
    }

    if value {
        value_func(a, c, r, pos_equalto, pos_end, arr, graph, formula_array, state)?;
    } else if arth_exp {
        arth_op(a, c, r, pos_equalto, pos_end, arr, graph, formula_array, state)?;
    } else if func {
        let func_name = &a[pos_equalto + 1..a[pos_equalto..].find('(').unwrap() + pos_equalto];
        match func_name {
            "MIN" => range_func(a, c, r, pos_equalto, pos_end, arr, graph, formula_array, state, 9)?,
            "MAX" => range_func(a, c, r, pos_equalto, pos_end, arr, graph, formula_array, state, 10)?,
            "AVG" => range_func(a, c, r, pos_equalto, pos_end, arr, graph, formula_array, state, 11)?,
            "SUM" => range_func(a, c, r, pos_equalto, pos_end, arr, graph, formula_array, state, 12)?,
            "STDEV" => range_func(a, c, r, pos_equalto, pos_end, arr, graph, formula_array, state, 13)?,
            "SLEEP" => sleep_func(a, c, r, pos_equalto, pos_end, arr, graph, formula_array, state)?,
            _ => return Err("Unknown function"),
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_parser() {
        assert_eq!(cell_parser("A1", 10, 10, 0, 2).unwrap(), 0);
        assert_eq!(cell_parser("B2", 10, 10, 0, 2).unwrap(), 11);
        assert!(cell_parser("Z1", 10, 10, 0, 2).is_err());
    }
}