use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};

use crate::server_models::Sheet;
use sheet::function_ext::Cell;
use sheet::graph_ext::{Graph, Formula, State, StateSnapshot};

// Create an enhanced AppState that includes all the extended functionality
pub struct ExtendedState {
    pub sheet: Sheet,                        // For API compatibility with existing endpoints
    pub cells: Vec<Cell>,                    // Typed cells (int, float, string)
    pub formula_array: Vec<Formula>,         // Cell formulas
    pub graph: Graph,                        // Dependency graph
    pub state: State,                        // Global sheet state
    pub undo_stack: Vec<StateSnapshot>,      // For undo operations
    pub redo_stack: Vec<StateSnapshot>,      // For redo operations
    pub current_x: i32,                      // Current cursor X
    pub current_y: i32,                      // Current cursor Y
}

// Create a shared state for the application
pub type AppState = Arc<RwLock<ExtendedState>>;

// Define request payload for updating a cell
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateCellRequest {
    pub row_id: String,
    pub column_id: String,
    pub value: String,
}

// Define response for update operations
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateResponse {
    pub success: bool,
    pub message: String,
}

// Define response for query operations
#[derive(Serialize, Deserialize, Debug)]
pub struct QueryResponse {
    pub success: bool,
    pub message: String,
    pub result: Option<String>,
}

// Add undo/redo specific types
#[derive(Serialize, Deserialize, Debug)]
pub struct UndoRedoResponse {
    pub success: bool,
    pub message: String,
}