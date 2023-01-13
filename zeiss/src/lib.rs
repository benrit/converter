
use std::fs;
use std::path::Path;

use std::collections::HashMap;

use shared::drivers::config::{Config};
use shared::drivers::elements;



fn read_chr_file(filename: &Path) -> Option<Vec<elements::ChrData>> {
    let file = match fs::read_to_string(filename) {
        Ok(data) => { data },
        Err(_) => {return None},
    };

    let data: Vec<&str> = file.split("\r\n").collect();
    let header_str = data[0];
    let header_map = header_str.split("\t").enumerate().map(|f| (f.1, f.0));
    let hm: HashMap<&str, usize> = header_map.into_iter().collect();

    let mut chr_data: Vec<elements::ChrData> = Vec::new();

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


            let chr = elements::ChrData{ 
                id: id.to_string(), 
                id_type: id_type.to_string(), 
                act: act, 
                nom: nom, 
                utol: utol, 
                ltol: ltol, 
                f_id: f_id.to_string(), 
                group: group.to_string(), 
                groups: groups.to_string() };

            chr_data.push(chr);
        }
    }

    Some(chr_data)
}

pub fn convert(config: &mut Config) {

    let files = fs::read_dir(config.configuation.machine_result_file.to_owned()).unwrap();

    config.chr_data = Some(Vec::new());


    for file in files{

        match file.unwrap().path().to_str().unwrap() {
            
            c if c.contains("chr") => { 
                config.chr_data = read_chr_file(Path::new(c));
            },
            c if c.contains("fet") => {
                    
            }
            _=>{}
        }
    }
}
