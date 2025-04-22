use operational_transform::OperationSeq;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

pub mod utils;

#[wasm_bindgen]
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct OpSeq(OperationSeq);

#[wasm_bindgen]
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct OpSeqPair(OpSeq, OpSeq);

impl OpSeq {
    
}