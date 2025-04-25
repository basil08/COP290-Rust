//! # Spreadsheet Operations
//! 
//! This module contains utility functions for performing operations on
//! spreadsheet cells and ranges, such as calculating sums, averages,
//! and clearing cell contents.

use crate::types::{AppState, QueryResponse};
use axum::Json;

use sheet::function_ext::{Cell, CellValue};

use crate::server_models::Sheet;

/// Converts a cell value to its string representation.
///
/// # Arguments
///
/// * `cell` - The cell value to convert
///
/// # Returns
///
/// A string representation of the cell value
pub fn cell_to_string(cell: &CellValue) -> String {
    match cell {
        CellValue::Int(i) => i.to_string(),
        CellValue::Float(f) => f.to_string(),
        CellValue::String(s) => s.clone(),
    }
}

/// Parses cell coordinates from a query string.
///
/// Expects format: "row,col" where both are integers
///
/// # Arguments
///
/// * `coord_str` - String containing the cell coordinates
///
/// # Returns
///
/// A tuple containing (row, col) indices or an error message
pub fn parse_cell_coordinates(coord_str: &str) -> Result<(usize, usize), &'static str> {
    let parts: Vec<&str> = coord_str.split(',').collect();
    if parts.len() != 2 {
        return Err("Invalid cell coordinate format");
    }

    let row = parts[0].trim().parse::<usize>().map_err(|_| "Invalid row")?;
    let col = parts[1].trim().parse::<usize>().map_err(|_| "Invalid column")?;

    Ok((row, col))
}

/// Calculates the sum of cells in a specified range.
///
/// # Arguments
///
/// * `state` - Application state containing the sheet data
/// * `query` - Query string specifying the range to sum
///
/// # Returns
///
/// A JSON response containing the sum result
pub async fn calculate_sum(state: &AppState, query: &str) -> Json<QueryResponse> {
    Json(QueryResponse {
        success: true,
        message: format!("TODO {}", query),
        result: Some("TODO".to_string()),
    })
}

/// Calculates the average of cells in a specified range.
///
/// # Arguments
///
/// * `state` - Application state containing the sheet data
/// * `query` - Query string specifying the range to average
///
/// # Returns
///
/// A JSON response containing the average result
pub async fn calculate_average(state: &AppState, query: &str) -> Json<QueryResponse> {
    Json(QueryResponse {
        success: true,
        message: format!("TODO {}", query),
        result: Some("TODO".to_string()),
    })
}

/// Clears cells in a specified range or the entire sheet.
///
/// If only "clear" is provided without coordinates, all cells are cleared.
/// Otherwise, only the specified range is cleared.
///
/// # Arguments
///
/// * `state` - Application state containing the sheet data
/// * `query` - Query string specifying what to clear
///
/// # Returns
///
/// A JSON response indicating how many cells were cleared
pub async fn clear_cells(state: &AppState, query: &str) -> Json<QueryResponse> {
    let parts: Vec<&str> = query.split_whitespace().collect();

    // Special case: if the query is just "clear" (without coordinates), clear the entire sheet
    if parts.len() == 1 {
        let mut sheet_state = state.write().await;
        let mut cleared_count = 0;

        for row in sheet_state.sheet.data.iter_mut() {
            for cell in row.iter_mut() {
                if !cell_to_string(&cell.value).is_empty() {
                    cell.value = CellValue::String("0".to_string());
                    cleared_count += 1;
                }
            }
        }
        return Json(QueryResponse {
            success: true,
            message: format!("Cleared all cells ({} cells cleared)", cleared_count),
            result: Some(cleared_count.to_string()),
        });
    } else {
        Json(QueryResponse {
            success: true,
            message: format!("TODO {}", query),
            result: Some("TODO".to_string()),
        })
    }
}

/// Counts non-empty cells in a specified range.
///
/// # Arguments
///
/// * `state` - Application state containing the sheet data
/// * `query` - Query string specifying the range to count
///
/// # Returns
///
/// A JSON response containing the count result
pub async fn count_cells(state: &AppState, query: &str) -> Json<QueryResponse> {
    Json(QueryResponse {
        success: true,
        message: format!("TODO {}", query),
        result: Some("TODO".to_string()),
    })
}