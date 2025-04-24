use serde::{Deserialize, Serialize};
use sheet::function_ext::{Cell, CellValue};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sheet {
    pub data: Vec<Vec<Cell>>,
}

impl Sheet {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            data: vec![vec![Cell::new_int(0); cols]; rows],
        }
    }
    pub fn get_data(&self) -> &Vec<Vec<Cell>> {
        &self.data
    }
}


