#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use toml::value::{Array};


use super::elements::HType;


#[allow(nonstandard_style)]
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct Dialog {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub partID: String,
    #[serde(default)]
    pub MSN: String,
    #[serde(default)]
    pub CAV: String,
    #[serde(default)]
    pub xOffset: String,
    #[serde(default)]
    pub yOffset: String,
    #[serde(default)]
    pub zOffset: String,
    #[serde(default)]
    pub WO: String,
    #[serde(default)]
    pub SO: String,
    #[serde(default)]
    pub comment: String,
    #[serde(default)]
    pub startRun: f64,
    #[serde(default)]
    pub endRun: f64,
    #[serde(default)]
    pub operation: String,
    #[serde(default)]
    pub tags: String,
    #[serde(default)]
    pub status: Option<String>,
    pub counter: Option<u32>
}

#[allow(nonstandard_style)]
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct Setup {
    #[serde(default)]
    pub nominalXoffset: String,
    #[serde(default)]
    pub nominalYoffset: String,
    #[serde(default)]
    pub nominalZoffset: String,
    #[serde(default)]
    pub fileIndex: String,
    #[serde(default)]
    pub importScan: bool,
    #[serde(default)]
    pub autorun: bool,
}

#[allow(nonstandard_style)]
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct DialogJson{
    pub Dialog: Dialog,
    pub Setup: Setup,
    #[serde(default)]
    pub Export: String
}

#[derive(Deserialize, Serialize,Debug, PartialEq, Clone)]

pub struct Machine{
    #[serde(default, alias = "type")]
    pub machine_type: String,
    pub dmesn: String,
    pub dmeswv: String,
    pub controllertyp: String,
    pub dmeswi: String,
    pub dmeid: String
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct Database{
    target: String,
    uri: String
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct Configuation{
    pub machine_result_path: String, //results from the machine
    pub cmm_result_path: String, // export to this path location
    pub xml_result_path: String, //legacy path format
    pub database: Option<Database>
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Actions{
    pub pipeline: Option<Array>
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct Export {
    #[serde(alias="type")]
    ty: String
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct Config{

    pub machine: Machine,
    pub configuation: Configuation,
    // pub actions: Option<Actions>,
    pub part_id: Option<String>,
    pub part_nb: Option<String>,
    #[serde(default, alias="chrData")]
    pub chr_data: Option<Vec<HType>>,
    #[serde(default, alias="fetData")]
    pub fet_data: Option<Vec<HType>>,
    // pub fet_data: Option<Vec<FetElement>>
    pub dialog_data: Option<DialogJson>
}

