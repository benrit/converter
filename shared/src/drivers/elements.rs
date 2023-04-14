#![allow(dead_code, non_snake_case)]

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

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct Point{
    // pub seq: usize,
    // pub id: String,
    // pub idType: String,
    // pub nomx: f64,
    // pub nomy: f64,
    // pub nomz: f64,
    // pub actx: f64,
    // pub acty: f64,
    // pub actz: f64,
    // pub i: f64,
    // pub j: f64,
    // pub k: f64,
}