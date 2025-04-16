pub mod display;
pub mod function;
pub mod graph;
pub mod parser;
pub mod util;


// use crate::{ parser, util, display, graph};

pub use function::{Cell, CellValue}; // make Cell public


pub fn generate_grid(rows: usize, cols: usize) -> Vec<Vec<Cell>> {
    (0..rows).map(|_r| {
        (0..cols).map(|_c| {
            let mut cell = Cell::default();
            cell.is_valid = true;
            cell.value = CellValue::Int(0);  // Initialize all cells to zero
            cell
        }).collect()
    }).collect()
}
