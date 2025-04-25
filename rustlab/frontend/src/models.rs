//! # Data Models
//!
//! This module defines the core data structures used in the spreadsheet application.
//! It provides a representation of the sheet and its cells that can be serialized
//! and deserialized for communication with the backend.

use serde::{Deserialize, Serialize};
use sheet::function_ext::{Cell, CellValue};

/// Represents a spreadsheet with rows and columns of cells.
///
/// This structure holds the entire grid of cells that make up the spreadsheet.
/// It can be serialized for API communication and provides methods for creating
/// and accessing the spreadsheet data.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sheet {
    /// The two-dimensional grid of cells that stores the spreadsheet data.
    pub data: Vec<Vec<Cell>>,
}

impl Sheet {
    /// Creates a new spreadsheet with the specified number of rows and columns.
    ///
    /// All cells are initialized with integer value 0.
    ///
    /// # Arguments
    ///
    /// * `rows` - The number of rows in the sheet
    /// * `cols` - The number of columns in the sheet
    ///
    /// # Returns
    ///
    /// A new `Sheet` instance with the specified dimensions
    pub fn new(rows: usize, cols: usize) -> Self {
        Self { data: vec![vec![Cell::new_int(0); cols]; rows] }
    }

    /// Returns a reference to the grid of cells.
    ///
    /// # Returns
    ///
    /// A reference to the two-dimensional vector of cells
    pub fn get_data(&self) -> &Vec<Vec<Cell>> {
        &self.data
    }
}