//! # Request Handlers
//! 
//! This module contains handler functions for the various API endpoints exposed by
//! the spreadsheet server. It implements the core functionality for retrieving sheet data,
//! updating cells, processing queries, and performing undo/redo operations.
#![deny(clippy::all)]

use axum::{
    body::Bytes,
    extract::{Json as ExtractJson, State},
    response::IntoResponse,
    Json,
};

use crate::server_models::Sheet;
use crate::types::{AppState, QueryResponse, UpdateCellRequest, UpdateResponse};

use sheet::graph_ext::State as State1;
use sheet::parser_ext::*;

use crate::create_snapshot;
use crate::types::UndoRedoResponse;

use sheet::{
    function_ext::{Cell, CellValue},
    graph_ext::{Formula, Graph},
};

/// Retrieves the current spreadsheet data.
///
/// This handler returns the complete sheet data in a format suitable for 
/// client-side rendering.
///
/// # Arguments
///
/// * `state` - Application state containing the current sheet
///
/// # Returns
///
/// The current spreadsheet as a JSON response
pub async fn get_sheet(state: State<AppState>) -> Json<Sheet> {
    let sheet = state.read().await.sheet.clone();
    Json(sheet)
}

/// Handles requests to undo the last action.
///
/// This handler:
/// 1. Removes the most recent state from the undo stack
/// 2. Saves current state to the redo stack
/// 3. Restores the previous state from the undo stack
///
/// # Arguments
///
/// * `state` - Application state containing undo/redo stacks and sheet data
///
/// # Returns
///
/// A JSON response indicating success or failure of the undo operation
pub async fn undo_action(State(state): State<AppState>) -> Json<UndoRedoResponse> {
    let mut app_state = state.write().await;

    if let Some(prev) = app_state.undo_stack.pop() {
        // Create temporary copies for snapshot creation
        let cells_copy = app_state.cells.clone();
        let formula_array_copy = app_state.formula_array.clone();
        let graph_copy = app_state.graph.clone();

        // Save current state to redo stack before reverting
        app_state.redo_stack.push(create_snapshot(&cells_copy, &formula_array_copy, &graph_copy));

        // Restore previous state
        app_state.cells = prev.arr;
        app_state.formula_array = prev.formula_array;
        app_state.graph = prev.graph;

        // Also update the regular sheet model for API compatibility
        let cells_clone = app_state.cells.clone();
        update_simple_sheet_from_cells(&mut app_state.sheet, &cells_clone, 10, 10);

        Json(UndoRedoResponse { success: true, message: "Action undone successfully".to_string() })
    } else {
        Json(UndoRedoResponse { success: false, message: "Nothing to undo".to_string() })
    }
}

/// Handles requests to redo a previously undone action.
///
/// This handler:
/// 1. Removes the most recent state from the redo stack
/// 2. Saves current state to the undo stack
/// 3. Restores the next state from the redo stack
///
/// # Arguments
///
/// * `state` - Application state containing undo/redo stacks and sheet data
///
/// # Returns
///
/// A JSON response indicating success or failure of the redo operation
pub async fn redo_action(State(state): State<AppState>) -> Json<UndoRedoResponse> {
    let mut app_state = state.write().await;

    if let Some(next) = app_state.redo_stack.pop() {
        // Create temporary copies for snapshot creation
        let cells_copy = app_state.cells.clone();
        let formula_array_copy = app_state.formula_array.clone();
        let graph_copy = app_state.graph.clone();

        // Save current state to undo stack before redoing
        app_state.undo_stack.push(create_snapshot(&cells_copy, &formula_array_copy, &graph_copy));

        // Restore next state
        app_state.cells = next.arr;
        app_state.formula_array = next.formula_array;
        app_state.graph = next.graph;

        let cells_clone = app_state.cells.clone();
        // Also update the regular sheet model for API compatibility
        update_simple_sheet_from_cells(&mut app_state.sheet, &cells_clone, 10, 10);

        Json(UndoRedoResponse { success: true, message: "Action redone successfully".to_string() })
    } else {
        Json(UndoRedoResponse { success: false, message: "Nothing to redo".to_string() })
    }
}

/// Updates the simple sheet model from internal cell data.
///
/// This helper function synchronizes the sheet view model with the underlying cell data.
///
/// # Arguments
///
/// * `sheet` - The sheet model to update
/// * `cells` - Source cell data
/// * `rows` - Number of rows in the sheet
/// * `cols` - Number of columns in the sheet
fn update_simple_sheet_from_cells(sheet: &mut Sheet, cells: &[Cell], rows: usize, cols: usize) {
    for r in 0..rows {
        for c in 0..cols {
            let idx = r * cols + c;
            if idx < cells.len() {
                // Convert Cell to string and update the sheet
                match &cells[idx].value {
                    CellValue::Int(i) => sheet.data[r][c].value = CellValue::String(i.to_string()),
                    CellValue::Float(f) => {
                        sheet.data[r][c].value = CellValue::String(f.to_string())
                    }
                    CellValue::String(s) => sheet.data[r][c].value = CellValue::String(s.clone()),
                }
            }
        }
    }
}

/// Updates a specific cell in the spreadsheet.
///
/// This handler processes cell update requests from the client,
/// parsing and evaluating formulas if needed.
///
/// # Arguments
///
/// * `state` - Application state containing the current sheet
/// * `payload` - Update request containing row, column, and new value
///
/// # Returns
///
/// A JSON response indicating success or failure of the update operation
pub async fn update_cell(
    State(state): State<AppState>,
    ExtractJson(payload): ExtractJson<UpdateCellRequest>,
) -> Json<UpdateResponse> {
    // Parse row and column IDs from string to usize
    let row_index = match payload.row_id.parse::<usize>() {
        Ok(index) => index,
        Err(_) => {
            return Json(UpdateResponse {
                success: false,
                message: "Invalid row ID format".to_string(),
            })
        }
    };

    let col_index = match payload.column_id.parse::<usize>() {
        Ok(index) => index,
        Err(_) => {
            return Json(UpdateResponse {
                success: false,
                message: "Invalid column ID format".to_string(),
            })
        }
    };

    let mut app_state = state.write().await;
    let cols = app_state.sheet.data[0].len();

    // Calculate 1D index from row and column
    let cell_index = row_index * cols + col_index;

    // Check if the indices are valid
    if row_index >= app_state.sheet.data.len()
        || col_index >= app_state.sheet.data[0].len()
        || cell_index >= app_state.cells.len()
    {
        return Json(UpdateResponse {
            success: false,
            message: "Cell indices out of bounds".to_string(),
        });
    }

    // Try to parse the input value and determine its type
    if let Ok(int_val) = payload.value.parse::<i32>() {
        // It's an integer
        app_state.cells[cell_index] = Cell::new_int(int_val);
        app_state.sheet.data[row_index][col_index].value = CellValue::Int(int_val);
    } else if let Ok(float_val) = payload.value.parse::<f64>() {
        // It's a float
        app_state.cells[cell_index] = Cell::new_float(float_val);
        app_state.sheet.data[row_index][col_index].value = CellValue::Float(float_val);
    } else if payload.value.contains("=") {
        // It's a formula - parse and evaluate it
        let mut cells_clone = app_state.cells.clone();
        let mut formula_array_clone = app_state.formula_array.clone();
        let mut graph_clone = app_state.graph.clone();
        let mut state_clone = app_state.state.clone();
        match cell_parser(
            &payload.value,
            cols as i32,
            app_state.sheet.data.len() as i32,
            &mut cells_clone,
            &mut graph_clone,
            &mut formula_array_clone,
            &mut state_clone,
        ) {
            Ok(_) => {
                // Formula processed successfully
                // Update the actual state with the modified clones
                app_state.cells = cells_clone;
                app_state.formula_array = formula_array_clone;
                app_state.graph = graph_clone;
                app_state.state = state_clone;

                // Also update the display sheet
                match &app_state.cells[cell_index].value {
                    CellValue::Int(i) => {
                        app_state.sheet.data[row_index][col_index].value = CellValue::Int(*i)
                    }
                    CellValue::Float(f) => {
                        app_state.sheet.data[row_index][col_index].value = CellValue::Float(*f)
                    }
                    CellValue::String(s) => {
                        app_state.sheet.data[row_index][col_index].value =
                            CellValue::String(s.clone())
                    }
                }
            }
            Err(e) => {
                return Json(UpdateResponse {
                    success: false,
                    message: format!("Formula error: {}", e),
                });
            }
        }
    } else {
        // It's a string
        app_state.cells[cell_index] = Cell::new_string(payload.value.clone());
        app_state.sheet.data[row_index][col_index].value = CellValue::String(payload.value.clone());
    }

    // Return success response
    Json(UpdateResponse { success: true, message: "Cell updated successfully".to_string() })
}

/// Parses and evaluates a cell formula.
///
/// This function handles different types of formulas:
/// - Simple values (e.g., =10)
/// - Arithmetic expressions (e.g., =A1+B2)
/// - Functions (e.g., =SUM(A1:B3))
///
/// # Arguments
///
/// * `a` - The formula string
/// * `c` - Number of columns in the sheet
/// * `r` - Number of rows in the sheet
/// * `arr` - Array of cells to update
/// * `graph` - Dependency graph
/// * `formula_array` - Array of formulas
/// * `state` - Parser state
///
/// # Returns
///
/// Result indicating success or an error message
pub fn cell_parser(
    a: &str,
    c: i32,
    r: i32,
    arr: &mut [Cell],
    graph: &mut Graph,
    formula_array: &mut [Formula],
    state: &mut State1,
) -> Result<(), &'static str> {
    // Placeholder for the actual cell parser function
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
        print!(" [DEBUG] Value function: ");
        value_func(a, c, r, pos_equalto, pos_end, arr, graph, formula_array, state)?;
    } else if arth_exp {
        arth_op(a, c, r, pos_equalto, pos_end, arr, graph, formula_array, state)?;
    } else if func {
        let func_name = &a[pos_equalto + 1..a[pos_equalto..].find('(').unwrap() + pos_equalto];
        // println!("[DEBUG] Function name: {}", func_name);
        match func_name {
            "MIN" => {
                range_func(a, c, r, pos_equalto, pos_end, arr, graph, formula_array, state, 9)?
            }
            "MAX" => {
                range_func(a, c, r, pos_equalto, pos_end, arr, graph, formula_array, state, 10)?
            }
            "AVG" => {
                range_func(a, c, r, pos_equalto, pos_end, arr, graph, formula_array, state, 11)?
            }
            "SUM" => {
                range_func(a, c, r, pos_equalto, pos_end, arr, graph, formula_array, state, 12)?
            }
            "STDEV" => {
                range_func(a, c, r, pos_equalto, pos_end, arr, graph, formula_array, state, 13)?
            }
            "SLEEP" => sleep_func(a, c, r, pos_equalto, pos_end, arr, graph, formula_array, state)?,

            _ => return Err("Unknown function"),
        }
    }

    Ok(())
}

/// Processes query commands sent from the client.
///
/// This handler parses and executes commands like formulas,
/// functions, and other operations on the spreadsheet data.
///
/// # Arguments
///
/// * `state` - Application state containing the current sheet
/// * `body` - Raw request body containing the query string
///
/// # Returns
///
/// A JSON response with the result of the executed query
pub async fn process_query(State(state): State<AppState>, body: Bytes) -> impl IntoResponse {
    // Convert bytes to string
    let query_string = match String::from_utf8(body.to_vec()) {
        Ok(s) => s,
        Err(_) => {
            return Json(QueryResponse {
                success: false,
                message: "Failed to parse query string".to_string(),
                result: None,
            })
        }
    };

    let mut app_state = state.write().await;
    let cols = app_state.sheet.data[0].len();
    let rows = app_state.sheet.data.len();

    // Save current state for undo before modifying (for commands that modify state)
    let query = query_string.trim();

    let mut cells_clone = app_state.cells.clone();
    let mut formula_array_clone = app_state.formula_array.clone();
    let mut graph_clone = app_state.graph.clone();
    let mut state_clone = app_state.state.clone();
    // Process the query - for direct formula/command input
    match parser(
        query,
        cols as i32,
        rows as i32,
        &mut cells_clone,
        &mut graph_clone,
        &mut formula_array_clone,
        &mut state_clone,
    ) {
        Ok(_) => {
            // Formula processed successfully
            // Update the actual state with the modified clones
            app_state.cells = cells_clone;
            app_state.formula_array = formula_array_clone;
            app_state.graph = graph_clone;
            app_state.state = state_clone;

            // Also update the display sheet
            for r in 0..rows {
                for c in 0..cols {
                    let idx = r * cols + c;
                    match &app_state.cells[idx].value {
                        CellValue::Int(i) => app_state.sheet.data[r][c].value = CellValue::Int(*i),
                        CellValue::Float(f) => {
                            app_state.sheet.data[r][c].value = CellValue::Float(*f)
                        }
                        CellValue::String(s) => {
                            app_state.sheet.data[r][c].value = CellValue::String(s.clone())
                        }
                    }
                }
            }

            // Formula processed successfully - continue with the existing code
            Json(QueryResponse {
                success: true,
                message: "Formula executed successfully".to_string(),
                result: None,
            })
        }
        Err(e) => {
            Json(QueryResponse {
                success: false,
                message: format!("Formula error: {}", e),
                result: None,
            })
        }
    }
}