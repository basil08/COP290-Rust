use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cell {
    pub value: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sheet {
    pub data: Vec<Vec<Cell>>,
}
