use crate::types::{AppState, QueryResponse};
use axum::Json;

use sheet::function_ext::{Cell, CellValue};

use crate::server_models::Sheet;

pub fn cell_to_string(cell: &CellValue) -> String {
    match cell {
        CellValue::Int(i) => i.to_string(),
        CellValue::Float(f) => f.to_string(),
        CellValue::String(s) => s.clone(),
    }
}

// Helper function to parse cell coordinates from a query
pub fn parse_cell_coordinates(coord_str: &str) -> Result<(usize, usize), &'static str> {
    let parts: Vec<&str> = coord_str.split(',').collect();
    if parts.len() != 2 {
        return Err("Invalid cell coordinate format");
    }

    let row = parts[0]
        .trim()
        .parse::<usize>()
        .map_err(|_| "Invalid row")?;
    let col = parts[1]
        .trim()
        .parse::<usize>()
        .map_err(|_| "Invalid column")?;

    Ok((row, col))
}

// Helper function to calculate sum of cells in a range
pub async fn calculate_sum(state: &AppState, query: &str) -> Json<QueryResponse> {
    Json(QueryResponse {
        success: true,
        message: format!("TODO {}", query),
        result: Some("TODO".to_string()),
    })
}

// Helper function to calculate average of cells in a range
pub async fn calculate_average(state: &AppState, query: &str) -> Json<QueryResponse> {
    Json(QueryResponse {
        success: true,
        message: format!("TODO {}", query),
        result: Some("TODO".to_string()),
    })
}

// Helper function to clear cells in a range
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

// Helper function to count non-empty cells in a range
pub async fn count_cells(state: &AppState, query: &str) -> Json<QueryResponse> {
    Json(QueryResponse {
        success: true,
        message: format!("TODO {}", query),
        result: Some("TODO".to_string()),
    })
}
