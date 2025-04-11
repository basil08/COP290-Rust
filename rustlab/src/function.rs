// use std::f64;

// #[derive(Clone)]
// pub struct Cell {
//     pub value: CellValue,
//     pub is_valid: bool,
// }

// #[derive(Clone)]
// pub enum CellValue {
//     Int(i32),
//     Float(f64),
//     String(String),
// }

// impl Cell {
//     pub fn new_int(value: i32) -> Self {
//         Cell {
//             value: CellValue::Int(value),
//             is_valid: true,
//         }
//     }

//     pub fn new_float(value: f64) -> Self {
//         Cell {
//             value: CellValue::Float(value),
//             is_valid: true,
//         }
//     }

//     pub fn new_string(value: String) -> Self {
//         Cell {
//             value: CellValue::String(value),
//             is_valid: true,
//         }
//     }

//     pub fn invalid() -> Self {
//         Cell {
//             value: CellValue::Int(0),
//             is_valid: false,
//         }
//     }
// }

// fn error_return() -> i32 {
//     INVALID_RANGE.with(|cell| cell.set(true));
//     -1
// }

// fn validate_range(range_start: i32, range_end: i32, c: i32) -> bool {
//     let start_row = range_start / c;
//     let start_col = range_start % c;
//     let end_row = range_end / c;
//     let end_col = range_end % c;

//     !(start_row > end_row || (start_row == end_row && start_col > end_col))
// }

// fn arithmetic_eval(v1: Cell, v2: Cell, op: char) -> Cell {
//     if !v1.is_valid || !v2.is_valid {
//         return Cell::invalid();
//     }

//     match (&v1.value, &v2.value) {
//         (CellValue::Int(i1), CellValue::Int(i2)) => {
//             match op {
//                 '+' => Cell::new_int(i1 + i2),
//                 '-' => Cell::new_int(i1 - i2),
//                 '*' => Cell::new_int(i1 * i2),
//                 '/' => {
//                     if *i2 == 0 {
//                         Cell::invalid()
//                     } else {
//                         Cell::new_int(i1 / i2)
//                     }
//                 }
//                 _ => Cell::invalid(),
//             }
//         }
//         (CellValue::Float(f1), CellValue::Float(f2)) => {
//             match op {
//                 '+' => Cell::new_float(f1 + f2),
//                 '-' => Cell::new_float(f1 - f2),
//                 '*' => Cell::new_float(f1 * f2),
//                 '/' => {
//                     if *f2 == 0.0 {
//                         Cell::invalid()
//                     } else {
//                         Cell::new_float(f1 / f2)
//                     }
//                 }
//                 _ => Cell::invalid(),
//             }
//         }
//         (CellValue::Int(i1), CellValue::Float(f2)) => {
//             let f1 = *i1 as f64;
//             match op {
//                 '+' => Cell::new_float(f1 + f2),
//                 '-' => Cell::new_float(f1 - f2),
//                 '*' => Cell::new_float(f1 * f2),
//                 '/' => {
//                     if *f2 == 0.0 {
//                         Cell::invalid()
//                     } else {
//                         Cell::new_float(f1 / f2)
//                     }
//                 }
//                 _ => Cell::invalid(),
//             }
//         }
//         (CellValue::Float(f1), CellValue::Int(i2)) => {
//             let f2 = *i2 as f64;
//             match op {
//                 '+' => Cell::new_float(f1 + f2),
//                 '-' => Cell::new_float(f1 - f2),
//                 '*' => Cell::new_float(f1 * f2),
//                 '/' => {
//                     if f2 == 0.0 {
//                         Cell::invalid()
//                     } else {
//                         Cell::new_float(f1 / f2)
//                     }
//                 }
//                 _ => Cell::invalid(),
//             }
//         }
//         _ => Cell::invalid(),
//     }
// }

// fn return_optype(op: char) -> i32 {
//     match op {
//         '+' => 1,
//         '-' => 2,
//         '*' => 3,
//         '/' => 4,
//         _ => i32::MIN,
//     }
// }

// struct RangeInfo {
//     first_cell: i32,
//     range_start: i32,
//     range_end: i32,
//     start_row: i32,
//     start_col: i32,
//     end_row: i32,
//     end_col: i32,
// }

// fn parse_and_validate_range(
//     a: &str,
//     c: i32,
//     r: i32,
//     pos_equalto: usize,
//     pos_end: usize,
//     graph: &mut Graph,
//     formula_array: &mut Vec<Formula>,
//     formula_type: i32,
// ) -> Result<RangeInfo, i32> {
//     let first_cell = cell_parser(a, c, r, 0, pos_equalto - 1, graph);
//     if first_cell == -1 {
//         return Err(error_return());
//     }

//     let eq_str = &a[pos_equalto..];
//     let open_paren = eq_str.find('(').map(|i| i + pos_equalto);
//     let close_paren = eq_str.find(')').map(|i| i + pos_equalto);

//     if open_paren.is_none() || close_paren.is_none() || close_paren.unwrap() <= open_paren.unwrap() + 1 {
//         return Err(error_return());
//     }

//     let open_paren = open_paren.unwrap();
//     let close_paren = close_paren.unwrap();
//     let colon_pos = a[open_paren + 1..].find(':').map(|i| i + open_paren + 1);

//     if colon_pos.is_none() {
//         return Err(error_return());
//     }

//     let colon_pos = colon_pos.unwrap();
//     let range_start = cell_parser(a, c, r, open_paren + 1, colon_pos - 1, graph);
//     let range_end = cell_parser(a, c, r, colon_pos + 1, close_paren - 1, graph);

//     if range_start == -1 || range_end == -1 || !validate_range(range_start, range_end, c) {
//         return Err(error_return());
//     }

//     graph.add_formula(first_cell, range_start, range_end, formula_type, formula_array);
//     graph.add_range_to_graph(range_start, range_end, first_cell);

//     let start_row = range_start / c;
//     let start_col = range_start % c;
//     let end_row = range_end / c;
//     let end_col = range_end % c;

//     Ok(RangeInfo {
//         first_cell,
//         range_start,
//         range_end,
//         start_row,
//         start_col,
//         end_row,
//         end_col,
//     })
// }

// fn min_func(
//     a: &str,
//     c: i32,
//     r: i32,
//     pos_equalto: usize,
//     pos_end: usize,
//     arr: &mut [Cell],
//     graph: &mut Graph,
//     formula_array: &mut Vec<Formula>,
// ) -> i32 {
//     let range_info = match parse_and_validate_range(a, c, r, pos_equalto, pos_end, graph, formula_array, 9) {
//         Ok(info) => info,
//         Err(err) => return err,
//     };

//     let mut min_value: Option<f64> = None;

//     if range_info.start_row == range_info.end_row {
//         for idx in range_info.range_start..=range_info.range_end {
//             let cell = arr[idx as usize];
//             if !cell.is_valid {
//                 arr[range_info.first_cell as usize] = Cell::invalid();
//                 return 1;
//             }
//             let current_value = match cell.value {
//                 CellValue::Int(i) => i as f64,
//                 CellValue::Float(f) => f,
//                 CellValue::String(_) => {
//                     arr[range_info.first_cell as usize] = Cell::invalid();
//                     return 1;
//                 }
//             };
//             min_value = match min_value {
//                 None => Some(current_value),
//                 Some(min) => Some(min.min(current_value)),
//             };
//         }
//     } else {
//         for row in range_info.start_row..=range_info.end_row {
//             for col in range_info.start_col..=range_info.end_col {
//                 let idx = (row * c + col) as usize;
//                 let cell = arr[idx];
//                 if !cell.is_valid {
//                     arr[range_info.first_cell as usize] = Cell::invalid();
//                     return 1;
//                 }
//                 let current_value = match cell.value {
//                     CellValue::Int(i) => i as f64,
//                     CellValue::Float(f) => f,
//                     CellValue::String(_) => {
//                         arr[range_info.first_cell as usize] = Cell::invalid();
//                         return 1;
//                     }
//                 };
//                 min_value = match min_value {
//                     None => Some(current_value),
//                     Some(min) => Some(min.min(current_value)),
//                 };
//             }
//         }
//     }

//     arr[range_info.first_cell as usize] = match min_value {
//         Some(val) => {
//             if val.fract() == 0.0 {
//                 Cell::new_int(val as i32)
//             } else {
//                 Cell::new_float(val)
//             }
//         }
//         None => Cell::invalid(),
//     };
//     1
// }

// fn max_func(
//     a: &str,
//     c: i32,
//     r: i32,
//     pos_equalto: usize,
//     pos_end: usize,
//     arr: &mut [Cell],
//     graph: &mut Graph,
//     formula_array: &mut Vec<Formula>,
// ) -> i32 {
//     let range_info = match parse_and_validate_range(a, c, r, pos_equalto, pos_end, graph, formula_array, 10) {
//         Ok(info) => info,
//         Err(err) => return err,
//     };

//     let mut max_value: Option<f64> = None;

//     if range_info.start_row == range_info.end_row {
//         for idx in range_info.range_start..=range_info.range_end {
//             let cell = arr[idx as usize];
//             if !cell.is_valid {
//                 arr[range_info.first_cell as usize] = Cell::invalid();
//                 return 1;
//             }
//             let current_value = match cell.value {
//                 CellValue::Int(i) => i as f64,
//                 CellValue::Float(f) => f,
//                 CellValue::String(_) => {
//                     arr[range_info.first_cell as usize] = Cell::invalid();
//                     return 1;
//                 }
//             };
//             max_value = match max_value {
//                 None => Some(current_value),
//                 Some(max) => Some(max.max(current_value)),
//             };
//         }
//     } else {
//         for row in range_info.start_row..=range_info.end_row {
//             for col in range_info.start_col..=range_info.end_col {
//                 let idx = (row * c + col) as usize;
//                 let cell = arr[idx];
//                 if !cell.is_valid {
//                     arr[range_info.first_cell as usize] = Cell::invalid();
//                     return 1;
//                 }
//                 let current_value = match cell.value {
//                     CellValue::Int(i) => i as f64,
//                     CellValue::Float(f) => f,
//                     CellValue::String(_) => {
//                         arr[range_info.first_cell as usize] = Cell::invalid();
//                         return 1;
//                     }
//                 };
//                 max_value = match max_value {
//                     None => Some(current_value),
//                     Some(max) => Some(max.max(current_value)),
//                 };
//             }
//         }
//     }

//     arr[range_info.first_cell as usize] = match max_value {
//         Some(val) => {
//             if val.fract() == 0.0 {
//                 Cell::new_int(val as i32)
//             } else {
//                 Cell::new_float(val)
//             }
//         }
//         None => Cell::invalid(),
//     };
//     1
// }

// fn avg_func(
//     a: &str,
//     c: i32,
//     r: i32,
//     pos_equalto: usize,
//     pos_end: usize,
//     arr: &mut [Cell],
//     graph: &mut Graph,
//     formula_array: &mut Vec<Formula>,
// ) -> i32 {
//     let range_info = match parse_and_validate_range(a, c, r, pos_equalto, pos_end, graph, formula_array, 11) {
//         Ok(info) => info,
//         Err(err) => return err,
//     };

//     let mut sum: f64 = 0.0;
//     let mut count: i32 = 0;

//     for row in range_info.start_row..=range_info.end_row {
//         for col in range_info.start_col..=range_info.end_col {
//             let idx = (row * c + col) as usize;
//             let cell = arr[idx];
//             if !cell.is_valid {
//                 arr[range_info.first_cell as usize] = Cell::invalid();
//                 return 1;
//             }
//             sum += match cell.value {
//                 CellValue::Int(i) => i as f64,
//                 CellValue::Float(f) => f,
//                 CellValue::String(_) => {
//                     arr[range_info.first_cell as usize] = Cell::invalid();
//                     return 1;
//                 }
//             };
//             count += 1;
//         }
//     }

//     arr[range_info.first_cell as usize] = if count > 0 {
//         let avg = sum / count as f64;
//         if avg.fract() == 0.0 {
//             Cell::new_int(avg as i32)
//         } else {
//             Cell::new_float(avg)
//         }
//     } else {
//         Cell::invalid()
//     };
//     1
// }

// fn sum_func(
//     a: &str,
//     c: i32,
//     r: i32,
//     pos_equalto: usize,
//     pos_end: usize,
//     arr: &mut [Cell],
//     graph: &mut Graph,
//     formula_array: &mut Vec<Formula>,
// ) -> i32 {
//     let range_info = match parse_and_validate_range(a, c, r, pos_equalto, pos_end, graph, formula_array, 12) {
//         Ok(info) => info,
//         Err(err) => return err,
//     };

//     let mut sum: f64 = 0.0;

//     for row in range_info.start_row..=range_info.end_row {
//         for col in range_info.start_col..=range_info.end_col {
//             let idx = (row * c + col) as usize;
//             let cell = arr[idx];
//             if !cell.is_valid {
//                 arr[range_info.first_cell as usize] = Cell::invalid();
//                 return 1;
//             }
//             sum += match cell.value {
//                 CellValue::Int(i) => i as f64,
//                 CellValue::Float(f) => f,
//                 CellValue::String(_) => {
//                     arr[range_info.first_cell as usize] = Cell::invalid();
//                     return 1;
//                 }
//             };
//         }
//     }

//     arr[range_info.first_cell as usize] = if sum.fract() == 0.0 {
//         Cell::new_int(sum as i32)
//     } else {
//         Cell::new_float(sum)
//     };
//     1
// }

// fn stdev_func(
//     a: &str,
//     c: i32,
//     r: i32,
//     pos_equalto: usize,
//     pos_end: usize,
//     arr: &mut [Cell],
//     graph: &mut Graph,
//     formula_array: &mut Vec<Formula>,
// ) -> i32 {
//     let range_info = match parse_and_validate_range(a, c, r, pos_equalto, pos_end, graph, formula_array, 13) {
//         Ok(info) => info,
//         Err(err) => return err,
//     };

//     let mut values: Vec<f64> = Vec::new();
//     let mut count: i32 = 0;

//     for row in range_info.start_row..=range_info.end_row {
//         for col in range_info.start_col..=range_info.end_col {
//             let idx = (row * c + col) as usize;
//             let cell = arr[idx];
//             if !cell.is_valid {
//                 arr[range_info.first_cell as usize] = Cell::invalid();
//                 return 1;
//             }
//             let value = match cell.value {
//                 CellValue::Int(i) => i as f64,
//                 CellValue::Float(f) => f,
//                 CellValue::String(_) => {
//                     arr[range_info.first_cell as usize] = Cell::invalid();
//                     return 1;
//                 }
//             };
//             values.push(value);
//             count += 1;
//         }
//     }

//     if count <= 1 {
//         arr[range_info.first_cell as usize] = Cell::new_int(0);
//         return 1;
//     }

//     let mean = values.iter().sum::<f64>() / count as f64;
//     let variance = values.iter()
//         .map(|&x| (x - mean) * (x - mean))
//         .sum::<f64>() / count as f64;
//     let stdev = variance.sqrt();

//     arr[range_info.first_cell as usize] = if stdev.fract() == 0.0 {
//         Cell::new_int(stdev as i32)
//     } else {
//         Cell::new_float(stdev)
//     };
//     1
// }

// fn sleep_func(
//     a: &str,
//     c: i32,
//     r: i32,
//     pos_equalto: usize,
//     pos_end: usize,
//     arr: &mut [Cell],
//     graph: &mut Graph,
//     formula_array: &mut Vec<Formula>,
// ) -> i32 {
//     let target_cell = cell_parser(a, c, r, 0, pos_equalto - 1, graph);
//     if target_cell == -1 {
//         return error_return();
//     }

//     let eq_str = &a[pos_equalto..];
//     let open_paren = eq_str.find('(').map(|i| i + pos_equalto);
//     let close_paren = eq_str.find(')').map(|i| i + pos_equalto);

//     if open_paren.is_none() || close_paren.is_none() || close_paren.unwrap() <= open_paren.unwrap() + 1 {
//         return error_return();
//     }

//     let open_paren = open_paren.unwrap();
//     let close_paren = close_paren.unwrap();
//     let ref_cell = cell_parser(a, c, r, open_paren + 1, close_paren - 1, graph);

//     if ref_cell != -1 {
//         let sleep_value = arr[ref_cell as usize];
//         if !sleep_value.is_valid {
//             arr[target_cell as usize] = Cell::invalid();
//             graph.add_formula( target_cell, ref_cell, ref_cell, 14, formula_array); // Using ref_cell as a placeholder for range_end
//             graph.adjLists_head[ref_cell as usize] = graph.add_edge(target_cell, graph.adjLists_head[ref_cell as usize].clone());
//             return 1;
//         }
//         arr[target_cell as usize] = sleep_value.clone();
//         graph.add_formula(target_cell, ref_cell, ref_cell, 14, formula_array);
//         graph.adjLists_head[ref_cell as usize] = graph.add_edge(target_cell, graph.adjLists_head[ref_cell as usize].clone());
//     } else {
//         let sleep_str = &a[open_paren + 1..close_paren];
//         if let Ok(sleep_value) = sleep_str.parse::<i32>() {
//             arr[target_cell as usize] = Cell::new_int(sleep_value);
//             graph.add_formula(target_cell, target_cell, target_cell, 14, formula_array); // Using target_cell as placeholder
//         } else if let Ok(sleep_value) = sleep_str.parse::<f64>() {
//             arr[target_cell as usize] = Cell::new_float(sleep_value);
//             graph.add_formula(target_cell, target_cell, target_cell, 14, formula_array);
//         } else {
//             arr[target_cell as usize] = Cell::new_string(sleep_str.to_string());
//             graph.add_formula(target_cell, target_cell, target_cell, 14, formula_array);
//         }
//     }

//     1
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::graph::{Graph, Formula}; // Make sure graph module is accessible

//     fn create_test_graph_and_formula_array() -> (Graph, Vec<Formula>) {
//         (Graph::new(), Vec::new())
//     }

//     fn cell_int(v: i32) -> Cell {
//         Cell::new_int(v)
//     }

//     fn cell_float(v: f64) -> Cell {
//         Cell::new_float(v)
//     }

//     #[test]
//     fn test_min_func() {
//         let c = 3;
//         let r = 3;
//         let mut arr = vec![
//             cell_int(10), cell_int(20), cell_int(5),
//             cell_int(40), cell_int(30), cell_int(25),
//             cell_int(15), cell_int(35), cell_int(45),
//         ];
//         let formula = "A1=MIN(A1:A3)";
//         let (mut graph, mut formula_array) = create_test_graph_and_formula_array();

//         let result = min_func(formula, c, r, 2, formula.len(), &mut arr, &mut graph, &mut formula_array);
//         assert_eq!(result, 1);
//         assert_eq!(arr[0].value, CellValue::Int(10));
//     }

//     #[test]
//     fn test_avg_func() {
//         let c = 3;
//         let r = 3;
//         let mut arr = vec![
//             cell_int(10), cell_int(20), cell_int(30),
//             cell_int(40), cell_int(50), cell_int(60),
//             cell_int(70), cell_int(80), cell_int(90),
//         ];
//         let formula = "A1=AVG(A2:A4)";
//         let (mut graph, mut formula_array) = create_test_graph_and_formula_array();

//         let result = avg_func(formula, c, r, 2, formula.len(), &mut arr, &mut graph, &mut formula_array);
//         assert_eq!(result, 1);
//         assert_eq!(arr[0].value, CellValue::Int(40)); // avg(20+30+70)/3 = 40
//     }

//     #[test]
//     fn test_sum_func() {
//         let c = 3;
//         let r = 3;
//         let mut arr = vec![
//             cell_int(1), cell_int(2), cell_int(3),
//             cell_int(4), cell_int(5), cell_int(6),
//             cell_int(7), cell_int(8), cell_int(9),
//         ];
//         let formula = "A1=SUM(A2:C3)";
//         let (mut graph, mut formula_array) = create_test_graph_and_formula_array();

//         let result = sum_func(formula, c, r, 2, formula.len(), &mut arr, &mut graph, &mut formula_array);
//         assert_eq!(result, 1);
//         assert_eq!(arr[0].value, CellValue::Int(44)); // sum of A2 to C3
//     }

//     #[test]
//     fn test_stdev_func() {
//         let c = 2;
//         let r = 2;
//         let mut arr = vec![
//             cell_int(2), cell_int(4),
//             cell_int(4), cell_int(4),
//         ];
//         let formula = "A1=STDEV(A2:B3)";
//         let (mut graph, mut formula_array) = create_test_graph_and_formula_array();

//         let result = stdev_func(formula, c, r, 2, formula.len(), &mut arr, &mut graph, &mut formula_array);
//         assert_eq!(result, 1);
//         // Expected stdev ≈ 0.7071, so a float value
//         match arr[0].value {
//             CellValue::Float(f) => assert!((f - 0.7071).abs() < 0.01),
//             _ => panic!("Expected float"),
//         }
//     }
// }

// fn main() {
//     let mut arr = vec![
//         Cell::new_int(5), Cell::new_int(10), Cell::new_int(3),
//         Cell::new_int(7), Cell::new_int(8), Cell::new_int(6),
//         Cell::new_int(9), Cell::new_int(4), Cell::new_int(2),
//     ];

//     let mut graph = Graph::new();
//     let mut formula_array = Vec::new();
//     let formula = "A1=MIN(A2:A4)";
//     let c = 3;
//     let r = 3;

//     let result = min_func(formula, c, r, 2, formula.len(), &mut arr, &mut graph, &mut formula_array);
//     println!("Result: {}, A1: {:?}", result, arr[0]);
// }
use std::f64;

#[derive(Clone)]
pub struct Cell {
    pub value: CellValue,
    pub is_valid: bool,
}

#[derive(Clone)]
pub enum CellValue {
    Int(i32),
    Float(f64),
    String(String),
}

impl Cell {
    pub fn new_int(value: i32) -> Self {
        Cell {
            value: CellValue::Int(value),
            is_valid: true,
        }
    }

    pub fn new_float(value: f64) -> Self {
        Cell {
            value: CellValue::Float(value),
            is_valid: true,
        }
    }

    pub fn new_string(value: String) -> Self {
        Cell {
            value: CellValue::String(value),
            is_valid: true,
        }
    }

    pub fn invalid() -> Self {
        Cell {
            value: CellValue::Int(0),
            is_valid: false,
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Cell::invalid()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
            assert_eq!(f, 3.14);
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
}