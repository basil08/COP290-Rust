use serde::{Deserialize, Serialize};
use std::f64;
/// Represents a single cell in a spreadsheet.
///
/// Each cell holds a typed value (`Int`, `Float`, or `String`) and a flag indicating whether it's valid.
/// Invalid cells are used to represent errors (e.g., formula issues, range errors).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Cell {
    /// The actual value of the cell.
    pub value: CellValue,
    /// Indicates whether the cell's value is valid.
    pub is_valid: bool,
}

/// Represents the value stored in a `Cell`.
///
/// A cell can contain either an integer, a floating-point number, or a string.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CellValue {
    /// Integer value
    Int(i32),
    /// Floating-point value
    Float(f64),
    /// String value
    String(String),
}

impl Cell {
    /// Creates a new `Cell` containing an integer value.
    ///
    /// # Arguments
    /// * `value` - The integer to store in the cell.
    pub fn new_int(value: i32) -> Self {
        Cell { value: CellValue::Int(value), is_valid: true }
    }

    /// Creates a new `Cell` containing a floating-point value.
    ///
    /// # Arguments
    /// * `value` - The float to store in the cell.
    pub fn new_float(value: f64) -> Self {
        Cell { value: CellValue::Float(value), is_valid: true }
    }

    /// Creates a new `Cell` containing a string value.
    ///
    /// # Arguments
    /// * `value` - The string to store in the cell.
    pub fn new_string(value: String) -> Self {
        Cell { value: CellValue::String(value), is_valid: true }
    }

    /// Returns an invalid `Cell` instance.
    ///
    /// This is typically used when parsing fails or a computation is invalid.
    pub fn invalid() -> Self {
        Cell { value: CellValue::Int(0), is_valid: false }
    }
}

impl Default for Cell {
    /// Returns the default value for a `Cell`, which is an invalid cell.
    fn default() -> Self {
        Cell::invalid()
    }
}
