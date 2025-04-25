//! # Rust Spreadsheet Backend Server
//!
//! This module provides the HTTP server implementation for the Rust Spreadsheet application.
//! It handles API endpoints for spreadsheet operations, state management, and client communication.

mod handlers;
mod server_models;
mod types;

use axum::{
    routing::{get, post},
    Router,
};
use sheet::function_ext::Cell;
use sheet::graph_ext::StateSnapshot;
use sheet::graph_ext::{Formula, Graph, State};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};

use handlers::{get_sheet, process_query, redo_action, undo_action, update_cell};
use server_models::Sheet;
use types::ExtendedState;

/// Creates a snapshot of the current spreadsheet state for undo/redo functionality.
///
/// This function captures all data necessary to restore the sheet to a previous state,
/// including cells, formulas, and dependency graph.
///
/// # Arguments
///
/// * `arr` - The array of cells representing the current sheet data
/// * `formula_array` - The array of formulas associated with cells
/// * `graph` - The dependency graph tracking relationships between cells
///
/// # Returns
///
/// A `StateSnapshot` containing clones of all state components
fn create_snapshot(arr: &[Cell], formula_array: &[Formula], graph: &Graph) -> StateSnapshot {
    StateSnapshot {
        arr: arr.to_owned(),
        formula_array: formula_array.to_owned(),
        graph: graph.clone(),
    }
}

/// Application entry point - initializes and runs the HTTP server.
///
/// This function:
/// 1. Sets up the initial spreadsheet state
/// 2. Configures API endpoints and CORS policy
/// 3. Starts the HTTP server on the specified address
///
/// The server provides endpoints for:
/// - Getting the sheet data
/// - Updating individual cells
/// - Processing queries
/// - Undo/redo operations
#[tokio::main]
async fn main() {
    // Initialize the sheet with default values
    let r = 10;
    let c = 10;
    let num_cells = r * c;

    // Initialize extended state components
    let cells = vec![Cell::new_int(0); num_cells];

    // Initialize regular sheet model for API compatibility
    let sheet = Sheet::new(10, 10);
    let num_cells = r * c;

    let formula_array = vec![Formula::default(); num_cells];
    let graph = Graph::new(num_cells);
    let state = State::new();
    let undo_stack: Vec<StateSnapshot> = Vec::new();
    let redo_stack: Vec<StateSnapshot> = Vec::new();

    // Create the extended state with all components
    let extended_state = ExtendedState {
        sheet: sheet.clone(),
        cells: cells.clone(),
        formula_array: formula_array.clone(),
        graph: graph.clone(),
        state: state.clone(),
        undo_stack,
        redo_stack,
    };
    let app_state = Arc::new(RwLock::new(extended_state));

    // Create a CORS layer that allows any origin
    let cors = CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any);

    // Add the CORS layer to your router
    let app = Router::new()
        .route("/sheet", get(get_sheet))
        .route("/update-cell", post(update_cell))
        .route("/api/query", post(process_query))
        .route("/api/undo", post(undo_action)) // New endpoint for undo
        .route("/api/redo", post(redo_action)) // New endpoint for redo
        .with_state(app_state)
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("✅ Server running at http://{}", addr);
    println!("🔄 Cell update endpoint available at http://{}/update-cell", addr);
    println!("📝 Query endpoint available at http://{}/api/query", addr);
    println!("↩️ Undo endpoint available at http://{}/api/undo", addr);
    println!("↪️ Redo endpoint available at http://{}/api/redo", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app).await.unwrap();
}
