#![allow(dead_code)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub type HType = HashMap<String, Value>;

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct ChrItem{
    pub id: String,
    pub id_type: String,
    pub act: f64,
    pub nom: f64,
    pub utol: f64,
    pub ltol: f64,
    pub f_id: String,
    pub group: String,
    pub groups: String
}

