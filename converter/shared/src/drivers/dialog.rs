
use std::{fs, io};

use crate::drivers::config::DialogJson;


impl DialogJson{
    pub fn from_file(filename: &str)-> Option<Self>{
        let dialog_file = match fs::read_to_string(filename) {
            Ok(data) => {data},
            Err(_) => {return None},
        };
        let data: DialogJson = serde_json::from_str(dialog_file.as_str()).unwrap();
        Some(data)
    }

    pub fn to_file(&self, filename: &str) -> Result<(), io::Error>{
        let data = serde_json::to_string(self).unwrap();
        fs::write(filename, data)
    }
}