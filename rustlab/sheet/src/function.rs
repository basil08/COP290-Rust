//! # Spreadsheet Built-in Functions
//!
//! This module provides a set of core spreadsheet functions implemented in Rust. These functions operate
//! over a 2D spreadsheet represented as a flat 1D array of `i32` values. Each function reads a specified
//! rectangular range of cells, performs a calculation, and writes the result to a target cell.
//!
//! ## Supported Functions
//! - `min_func`: Minimum value in a range
//! - `max_func`: Maximum value in a range
//! - `sum_func`: Total sum of values in a range
//! - `avg_func`: Average (mean) of values in a range
//! - `stdev_func`: Standard deviation of values in a range
//! - `sleep_func`: Introduces a delay based on a cell value or a literal
//!
//! ## Architecture
//! - Spreadsheet is stored in a flat array (`Vec<i32>`) indexed as `row * cols + col`
//! - Dependency tracking is handled by a custom `Graph` structure
//! - Formulas are stored in a separate array and referenced by index
//! - Errors are tracked using a global `INVALID_RANGE` flag and sentinel value `i32::MIN`
//!
//! ## Example Usage
//! ```text
//! Input: "A1 = SUM(B1:B3)"
//! Parsed as: write result to cell A1, sum values from B1 to B3
//! ```

use crate::graph::{Formula, Graph};
use crate::parser::cell_parser;
use std::cmp::{max, min};
use std::i32;
use std::thread::sleep;
use std::time::Duration;
/// Global flag to indicate if a function encountered an invalid range.

pub static mut INVALID_RANGE: bool = false;

fn error_usize() -> usize {
    unsafe {
        INVALID_RANGE = true;
    }
    0
}

fn error_range() -> (usize, usize) {
    unsafe {
        INVALID_RANGE = true;
    }
    (0, 0)
}

fn error_return() -> i32 {
    unsafe {
        INVALID_RANGE = true;
    }
    -1
}
/// Validates that `start` to `end` defines a proper left-to-right, top-to-bottom rectangular range.

fn validate_range(start: usize, end: usize, cols: usize) -> bool {
    let (sr, sc) = (start / cols, start % cols);
    let (er, ec) = (end / cols, end % cols);
    sr < er || (sr == er && sc <= ec)
}
/// Calculates the standard deviation (rounded) from a slice of integers.

fn std(values: &[i32]) -> i32 {
    if values.len() <= 1 {
        return 0;
    }
    let mean = values.iter().sum::<i32>() as f64 / values.len() as f64;
    let variance = values
        .iter()
        .map(|&x| {
            let diff = x as f64 - mean;
            diff * diff
        })
        .sum::<f64>()
        / values.len() as f64;
    variance.sqrt().round() as i32
}
/// Parses a range of cells from a string (e.g., "SUM(B1:C3)") and returns the start and end indices.

fn extract_range_cells(
    a: &str,
    eq_idx: usize,
    c: usize,
    r: usize,
    graph: &mut Graph,
) -> Option<(usize, usize)> {
    let open_paren = a[eq_idx..].find('(')? + eq_idx;
    let close_paren = a[eq_idx..].find(')')? + eq_idx;
    if close_paren <= open_paren + 1 {
        return None;
    }
    let colon_pos = a[open_paren + 1..].find(':')? + open_paren + 1;
    let range_start = cell_parser(a, c, r, open_paren + 1, colon_pos - 1, graph)?;
    let range_end = cell_parser(a, c, r, colon_pos + 1, close_paren - 1, graph)?;
    if !validate_range(range_start, range_end, c) {
        return None;
    }
    Some((range_start, range_end))
}

/// Computes the minimum value within a specified range and stores it in the target cell.
/// Adds the formula and dependency to the graph for recalculation tracking.
/// Returns 1 on success.

pub fn min_func(
    a: &str,
    c: usize,
    r: usize,
    eq_idx: usize,
    _: usize,
    arr: &mut [i32],
    graph: &mut Graph,
    formula_array: &mut [Formula],
) -> i32 {
    // Get the first cell (target) for the formula
    let first_cell = cell_parser(a, c, r, 0, eq_idx - 1, graph).unwrap_or_else(error_usize);

    // Extract the range (start and end for both rows and columns)
    let (range_start, range_end) =
        extract_range_cells(a, eq_idx, c, r, graph).unwrap_or_else(error_range);

    // Add formula to the graph
    Graph::add_formula(graph, first_cell, range_start, range_end, 9, formula_array);
    graph.add_range_to_graph(range_start, range_end, first_cell);

    // Get the starting and ending row/column indices
    let (start_row, start_col) = (range_start / c, range_start % c);
    let (end_row, end_col) = (range_end / c, range_end % c);

    let mut min_val = arr[range_start];

    // Handle 1D range: If the range is a single column, loop only through rows
    if start_col == end_col {
        // 1D range (same column), loop through rows
        for row in start_row..=end_row {
            let idx = row * c + start_col; // Iterate through rows, same column
            min_val = min(min_val, arr[idx]);
        }
    } else {
        // 2D range (across multiple columns), loop through both rows and columns
        for row in start_row..=end_row {
            for col in start_col..=end_col {
                let idx = row * c + col; // Convert (row, col) to 1D index
                min_val = min(min_val, arr[idx]);
            }
        }
    }

    arr[first_cell] = min_val;
    1
}
/// Computes the maximum value within a specified range and stores it in the target cell.
/// Adds the formula and dependency to the graph for recalculation tracking.
/// Returns 1 on success.

pub fn max_func(
    a: &str,
    c: usize,
    r: usize,
    eq_idx: usize,
    _: usize,
    arr: &mut [i32],
    graph: &mut Graph,
    formula_array: &mut [Formula],
) -> i32 {
    // Get the first cell (target) for the formula
    let first_cell = cell_parser(a, c, r, 0, eq_idx - 1, graph).unwrap_or_else(error_usize);

    // Extract the 2D range (start and end for both rows and columns)
    let (range_start, range_end) =
        extract_range_cells(a, eq_idx, c, r, graph).unwrap_or_else(error_range);

    // Add formula to the graph
    Graph::add_formula(graph, first_cell, range_start, range_end, 10, formula_array);
    graph.add_range_to_graph(range_start, range_end, first_cell);

    // Determine 2D range bounds
    let (start_row, start_col) = (range_start / c, range_start % c);
    let (end_row, end_col) = (range_end / c, range_end % c);

    let mut max_value = arr[range_start];

    // Iterate over the 2D range
    for row in start_row..=end_row {
        for col in start_col..=end_col {
            let idx = row * c + col;
            // println!("idx: {}, arr[idx]: {}", idx, arr[idx]);
            max_value = max(max_value, arr[idx]);
        }
    }

    // Store the maximum value in the first cell
    arr[first_cell] = max_value;
    // println!("max_value: {}", max_value);
    // println!("arr[first_cell]: {}", arr[first_cell]);
    1
}

/// Computes the total sum of values within a specified range and stores it in the target cell.
/// Adds the formula and dependency to the graph for recalculation tracking.
/// Returns 1 on success.

pub fn sum_func(
    a: &str,
    c: usize,
    r: usize,
    eq_idx: usize,
    _: usize,
    arr: &mut [i32],
    graph: &mut Graph,
    formula_array: &mut [Formula],
) -> i32 {
    let first_cell = cell_parser(a, c, r, 0, eq_idx - 1, graph).unwrap_or_else(error_usize);
    let (range_start, range_end) =
        extract_range_cells(a, eq_idx, c, r, graph).unwrap_or_else(error_range);
    Graph::add_formula(graph, first_cell, range_start, range_end, 12, formula_array);
    graph.add_range_to_graph(range_start, range_end, first_cell);
    let sum: i32 = (range_start..=range_end).map(|idx| arr[idx]).sum();
    arr[first_cell] = sum;
    1
}
/// Computes the average value within a specified range and stores it in the target cell.
/// Adds the formula and dependency to the graph for recalculation tracking.
/// Returns 1 on success.

pub fn avg_func(
    a: &str,
    c: usize,
    r: usize,
    eq_idx: usize,
    _: usize,
    arr: &mut [i32],
    graph: &mut Graph,
    formula_array: &mut [Formula],
) -> i32 {
    let first_cell = cell_parser(a, c, r, 0, eq_idx - 1, graph).unwrap_or_else(error_usize);
    let (range_start, range_end) =
        extract_range_cells(a, eq_idx, c, r, graph).unwrap_or_else(error_range);

    Graph::add_formula(graph, first_cell, range_start, range_end, 11, formula_array);
    graph.add_range_to_graph(range_start, range_end, first_cell);

    let (start_row, start_col) = (range_start / c, range_start % c);
    let (end_row, end_col) = (range_end / c, range_end % c);

    let mut values = Vec::new();
    for row in start_row..=end_row {
        for col in start_col..=end_col {
            let idx = row * c + col;
            values.push(arr[idx]);
        }
    }

    let sum: i32 = values.iter().sum();
    let count = values.len();
    arr[first_cell] = if count > 0 { sum / count as i32 } else { 0 };
    1
}

/// Computes the standard deviation of values within a specified range and stores it in the target cell.
/// Adds the formula and dependency to the graph for recalculation tracking.
/// Returns 1 on success.

pub fn stdev_func(
    a: &str,
    c: usize,
    r: usize,
    eq_idx: usize,
    _: usize,
    arr: &mut [i32],
    graph: &mut Graph,
    formula_array: &mut [Formula],
) -> i32 {
    let first_cell = cell_parser(a, c, r, 0, eq_idx - 1, graph).unwrap_or_else(error_usize);
    let (range_start, range_end) =
        extract_range_cells(a, eq_idx, c, r, graph).unwrap_or_else(error_range);

    Graph::add_formula(graph, first_cell, range_start, range_end, 13, formula_array);
    graph.add_range_to_graph(range_start, range_end, first_cell);

    let (start_row, start_col) = (range_start / c, range_start % c);
    let (end_row, end_col) = (range_end / c, range_end % c);

    let mut values = Vec::new();
    for row in start_row..=end_row {
        for col in start_col..=end_col {
            let idx = row * c + col;
            values.push(arr[idx]);
        }
    }

    arr[first_cell] = std(&values);
    1
}
/// Delays execution for a number of seconds specified either directly or from a referenced cell.
/// Also stores the sleep duration in the target cell and logs the dependency.
/// Returns 1 on success.

pub fn sleep_func(
    a: &str,
    c: usize,
    r: usize,
    eq_idx: usize,
    _: usize,
    arr: &mut [i32],
    graph: &mut Graph,
    formula_array: &mut [Formula],
) -> i32 {
    let target_cell = cell_parser(a, c, r, 0, eq_idx - 1, graph).unwrap_or_else(error_usize);
    let open_paren = a[eq_idx..].find('(').map(|i| i + eq_idx).unwrap_or(0);
    let close_paren = a[eq_idx..]
        .find(')')
        .map(|i| i + eq_idx)
        .unwrap_or(a.len() - 1);
    if close_paren <= open_paren + 1 {
        // println!("Invalid sleep function syntax");
        return error_return();
    }
    // println!("1");
    if let Some(ref_cell) = cell_parser(a, c, r, open_paren + 1, close_paren - 1, graph) {
        let sleep_value = arr[ref_cell];
        // println!("Sleep value: {}", sleep_value);
        if sleep_value == i32::MIN {
            arr[target_cell] = i32::MIN;
            return 1;
        }
        graph.add_edge(ref_cell, target_cell);
        Graph::add_formula(
            graph,
            target_cell,
            ref_cell,
            sleep_value as usize,
            14,
            formula_array,
        );
        if sleep_value > 0 {
            // println!("Sleeping for {} seconds 1", sleep_value);
            // sleep(Duration::from_secs(sleep_value as u64));
        } else {
            // println!("Sleep value is less than 0, status = OK");
        }
        arr[target_cell] = sleep_value;
    } else {
        // println!("2");
        let value: i32 = a[open_paren + 1..close_paren].trim().parse().unwrap_or(-1);
        Graph::add_formula(
            graph,
            target_cell,
            target_cell,
            value as usize,
            14,
            formula_array,
        );
        arr[target_cell] = value;
        if value > 0 {
            // println!("Sleeping for {} seconds 2", value);
            // sleep(Duration::from_secs(value as u64));
        } else {
            // println!("Sleep value is less than 0, status = OK");
        }
    }
    1
}
