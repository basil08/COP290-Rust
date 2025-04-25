//! # Type Definitions
//!
//! This module defines the core data types used throughout the application,
//! including state management structures, request and response formats,
//! and API interfaces.

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::server_models::Sheet;
use sheet::function_ext::Cell;
use sheet::graph_ext::{Formula, Graph, State, StateSnapshot};

/// Enhanced state container for the spreadsheet application.
///
/// This structure maintains all aspects of the spreadsheet state, including:
/// - The rendered sheet data
/// - Cell values and formulas
/// - Dependency tracking
/// - History for undo/redo operations
/// - Current cursor position
pub struct ExtendedState {
    /// Sheet model for API compatibility with existing endpoints
    pub sheet: Sheet,
    /// Typed cells (int, float, string)
    pub cells: Vec<Cell>,
    /// Cell formulas
    pub formula_array: Vec<Formula>,
    /// Dependency graph for formula evaluation
    pub graph: Graph,
    /// Global sheet state
    pub state: State,
    /// Stack of previous states for undo operations
    pub undo_stack: Vec<StateSnapshot>,
    /// Stack of undone states for redo operations
    pub redo_stack: Vec<StateSnapshot>,
}

/// Thread-safe shared application state.
///
/// This type provides synchronized read/write access to the application state
/// that can be safely shared across async tasks and API handlers.
pub type AppState = Arc<RwLock<ExtendedState>>;

/// Request payload for updating a cell value.
///
/// Contains the cell coordinates and the new value to set.
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateCellRequest {
    /// Row identifier (zero-based index as string)
    pub row_id: String,
    /// Column identifier (zero-based index as string)
    pub column_id: String,
    /// New value to set in the cell
    pub value: String,
}

/// Response format for cell update operations.
///
/// Indicates whether the update was successful and provides a message.
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateResponse {
    /// Whether the operation was successful
    pub success: bool,
    /// Message describing the result or error
    pub message: String,
}

/// Response format for query operations.
///
/// Includes success status, descriptive message, and optional result data.
#[derive(Serialize, Deserialize, Debug)]
pub struct QueryResponse {
    /// Whether the query was processed successfully
    pub success: bool,
    /// Message describing the result or error
    pub message: String,
    /// Optional result value from the query
    pub result: Option<String>,
}

/// Response format for undo/redo operations.
///
/// Provides feedback about the success of history navigation.
#[derive(Serialize, Deserialize, Debug)]
pub struct UndoRedoResponse {
    /// Whether the undo/redo operation was successful
    pub success: bool,
    /// Message describing the result or error
    pub message: String,
}
