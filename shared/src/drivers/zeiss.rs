use serde_json::{json, Value};
use std::fs;
use std::path::Path;

use crate::drivers::config;
use std::collections::HashMap;


struct Chr_Item {
    id: String,
    id_type: String
}


fn read_chr_file(filename: &Path, item_list: Vec<&str>) -> Option<Value> {
    let file = fs::read_to_string(filename).unwrap();

    let data: Vec<&str> = file.split("\r\n").collect();
    let header_str = data[0];
    let header_map = header_str.split("\t").enumerate().map(|f| (f.1, f.0));
    let hm: HashMap<&str, usize> = header_map.into_iter().collect();

    let mut chr_data: Vec<Value> = Vec::new();

    for item in &data[1..] {
        let temp = item.split("\t").collect::<Vec<&str>>();
        if temp.len() > 34 {
            let id = temp[hm["id"]];
            let id_type = temp[hm["idsymbol"]];
            let nom = temp[hm["nominal"]].parse::<f64>().unwrap();
            let act = temp[hm["actual"]].parse::<f64>().unwrap();
            let utol = temp[hm["uppertol"]].parse::<f64>().unwrap();
            let ltol = temp[hm["lowertol"]].parse::<f64>().unwrap();
            let f_id = temp[hm["featureid"]];
            let group = temp[hm["groupname"]];   
            let groups = &temp[hm["group1"]..].join(",");

            let chr = json!({
                "id": id,
                "idType": id_type,
                "act": act,
                "nom": nom,
                "utol": utol,
                "ltol": ltol,
                "f_id": f_id,
                "group": group,
                "groups": groups
            });

            chr_data.push(chr.to_owned());
        }
    }

    Some(json!(chr_data))
}

fn read_table_file(filename: &Path): Vec<String>{
    let file = read_to_string(filename).ok();


}

pub fn convert(config: &mut config::Config) {

    let files = fs::read_dir(config.configuation.machine_result_file.to_owned()).unwrap();

    for file in files{

        match file.unwrap().path().to_str().unwrap() {
            
            c if c.contains("chr") { 
                config.chr_data = read_chr_file(Path::new(c));
            },
            c if c.contains("fet") => {
                    
            }
            _=>{}
        }
    }

}
