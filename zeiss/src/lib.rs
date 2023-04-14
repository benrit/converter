#![allow(non_snake_case, unused)]

use std::fs;
use std::path::Path;
use std::collections::{HashMap, BTreeMap};
use serde_json::{json, Value};
use shared::drivers::config::{Config};
use shared::drivers::elements::{ChrItem, Point, HType};

mod zeiss_test;



enum ValueType {
    String,
    Float,
    Array(usize),
    Seq
}



trait Element {

    fn new<const N: usize>(input: &str, hm: &HashMap<&str, usize>, array: [(&'static str, &'static str, ValueType); N])->Option<HType>{
        
        let split = input.split("\t").collect::<Vec<&str>>();

        let mut temp = HType::new();

        if split.len() < 35 {
            return None;
        }

        let mut elem:HType = HType::new();

        for item in array{
            let index = *hm.get(item.1).unwrap();

            match item.2 {
                ValueType::String => {
                    elem.insert(item.0.to_string(), json!(split[index])); 
                },
                ValueType::Float => {
                    elem.insert(item.0.to_string(), json!(split[index].parse::<f64>().unwrap()));
                },
                ValueType::Array(n) => {
                    let mut arr:Vec<&str> = Vec::new();
                    let index = index + 1;

                    for i in index..index+n{
                        match split.get(i) {
                            Some(x) => arr.push(*x),
                            None => {},
                        };
                    }
                    elem.insert(item.0.to_string(), json!(arr.join(".")));
                },
                ValueType::Seq => {
                    elem.insert(item.0.to_string(), json!(elem.len() + 1));
                }
                _ => {}
            }  
        };
        Some(elem)
    }

    fn from_str(input: &str, hm: &HashMap<&str, usize>)-> Option<HType>;
}


impl Element for Point {
    fn from_str(input: &str, hm: &HashMap<&str, usize>) -> Option<HType> {
        Self::new(input, hm, [
            ("id", "id", ValueType::String), 
            ("idType", "idsymbol", ValueType::String), 
            ("id", "id", ValueType::String), 
            ("actx", "x", ValueType::Float), 
            ("acty", "y", ValueType::Float), 
            ("actz", "z", ValueType::Float), 
            ("nomx", "x_nom", ValueType::Float), 
            ("nomy", "y_nom", ValueType::Float), 
            ("nomz", "z_nom", ValueType::Float), 
            ("i", "nx", ValueType::Float), 
            ("j", "ny", ValueType::Float), 
            ("k", "nz", ValueType::Float)
            ])

    }
}


impl Element for ChrItem {
    fn from_str(input: &str, hm: &HashMap<&str, usize>) -> Option<HType> {
        Self::new(input, hm, [
            ("idType", "idsymbol", ValueType::String), 
            ("id", "id", ValueType::String), 
            ("nom", "nominal", ValueType::Float), 
            ("act", "actual", ValueType::Float),
            ("utol", "uppertol", ValueType::Float),
            ("ltol", "lowertol", ValueType::Float), 
            ("group", "groupname", ValueType::String),
            ("groups", "group1", ValueType::Array(10))
            ])
    }
}

fn read_fet_file(filename: &Path) -> Option<Vec<HType>> {

    let file = match fs::read_to_string(filename) {
        Ok(data) => {data}
        Err(_) => {return None}
    };

    let data: Vec<&str> = file.split("\r\n").collect();
    let header_str = data[0];
    let header_map = header_str.split("\t").enumerate().map(|f| (f.1, f.0));
    let hm: HashMap<&str, usize> = header_map.into_iter().collect();
    let mut fetData:Vec<HType> = Vec::new();
    let arr = [("idType", "idsymbol"), ("id", "id")];

    for item in &data[1..]{
        let temp = item.split("\t").collect::<Vec<&str>>();

        match temp[hm["idsymbol"]] {
            "Point" => {}
            _=>{}
        } ;

    }

    None
}


fn read_fet_file_v2(filename: &Path)-> Option<Vec<HType>> {
    let mut temp: Vec<HType> = Vec::new();

    let fet = fs::read_to_string(filename).unwrap();
    let fetData:Vec<&str> = fet.split("\r\n").collect();

    let hm = create_hm(fetData[0]);

    for line in &fetData[1..]{
        let t:Vec<&str> = line.split("\t").collect();
        let idType = t[*hm.get("idType")?];
        match idType {
            "SpacePoint" | "Point" => {
                temp.push(Point::from_str(line, &hm)?)
            },

            _ => {}
        }
    }

    Some(temp)
}

// Hashmap that contains the header row of the zeiss table file, since the order could change in case there are some custome fields in the table file
fn create_hm(input: &str)->HashMap<&str, usize>{
    let header_map = input.split("\t").enumerate().map(|f| (f.1, f.0));
    header_map.into_iter().collect()
}

fn read_chr_file(filename: &Path) -> Option<Vec<ChrItem>> {
    let file = match fs::read_to_string(filename) {
        Ok(data) => { data },
        Err(_) => {return None},
    };

    let data: Vec<&str> = file.split("\r\n").collect();

    let hm = create_hm(data[0]);

    let mut chr_data: Vec<ChrItem> = Vec::new();

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


            let chr = ChrItem{ 
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


fn read_chr_file_v2(filename: &Path) -> Option<Vec<HType>>{
    let mut temp: Vec<HType> = Vec::new();

    let chr = fs::read_to_string(filename).unwrap();
    let chrData:Vec<&str> = chr.split("\r\n").collect();

    let hm = create_hm(chrData[0]);

    for line in &chrData[1..]{
        match ChrItem::from_str(*line, &hm) {
            Some(t) => temp.push(t),
            None => {},
        };
    }
    Some(temp)
}

pub fn convert(config: &mut Config) -> std::io::Result<()> {

    let files = fs::read_dir(config.configuation.machine_result_path.as_str())?;

    for file in files{

        match file.unwrap().path().to_str().unwrap() {
            
            c if c.contains("chr") => { 
                config.chr_data = read_chr_file_v2(Path::new(c));
            },
            c if c.contains("fet") => {
                config.fet_data = read_fet_file_v2(Path::new(c));
            }
            _=>{}
        }
    }

    let _scan_data = config.dialog_data.as_ref().unwrap().Setup.importScan;

    Ok(())
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::hash::Hash;
    
    use std::collections::hash_map::{RandomState, DefaultHasher};
    // use super::*;
    use std::{time::SystemTime, path::PathBuf};
    use std::env;
    use serde::ser::SerializeMap;
    use serde_json::{json, Value};
    use shared::drivers::elements::{ChrItem, HType};

    use crate::{Point, Element, create_hm};
    use std::fs::read_to_string;

    #[test]
    fn test_ChrItem() {

        let startTime = SystemTime::now();

        let hm = create_hm("planid	partnb	id	type	idsymbol	actual	nominal	uppertol	lowertol	deviation	exceed	warningLimitCF	featureid	featuresigma	comment	link	linkmode	mmc	useruppertol	userlowertol	fftphi	fftphiunit	zoneroundnessangle	groupname	groupname2	datumAid	datumBid	datumCid	natuppertolid	natlowertolid	decimalplaces	featurePosX	featurePosY	featurePosZ	unit	group1	group2	group3	group4	group5	group6	group7	group8	group9	group10	");

        let chrItem = ChrItem::from_str("00469339-Gate	815	Gate_16MM_1.Seal_1_X	X1	xValue	1.0000000	0.0000000	0.0050000	-0.0050000	0.0000000		100.0000000	Gate_16MM_1.Seal_1	0.0000000					0.0050000	-0.0050000				Gate_16MM_1	Gate_16MM_1.Seal_Dia				1	1	3	-39.0000000	6.0000000	-10.1350000	mm	Gate_16MM_1	Gate_16MM_1.Seal_Dia", &hm);


        println!("Duration: {}ms", startTime.elapsed().unwrap().as_millis());

        assert_eq!(
            json!(chrItem), 
            json!({"idType":"xValue","id":"Gate_16MM_1.Seal_1_X", "nom":0.0, "act":1.0, "ltol": -0.005, "utol": 0.005, "group": "Gate_16MM_1", "groups": "Gate_16MM_1.Seal_Dia"}));
    }


    #[test]
    fn test_load_chrfile() {

        let startTime = SystemTime::now();

        let currentPath = env::current_dir().unwrap();
        let mut chrFile =  PathBuf::new();
        chrFile.push("C:\\Users\\uqtsy\\Rust\\cmmTools\\test_data\\results\\test_chr.txt");
        println!("{chrFile:?}");
        
        let chr = read_to_string(chrFile).unwrap();
        let chrData:Vec<&str> = chr.split("\r\n").collect();

        let hm = create_hm(chrData[0]);

        for line in &chrData[1..]{

            let chrItem = ChrItem::from_str(*line, &hm);
            println!("{}", serde_json::to_string(&chrItem).unwrap());
        }
        println!("Duration: {}ms", startTime.elapsed().unwrap().as_millis());
    }

    #[test]
    fn test_hashmap() {
        let hm = create_hm("planid	partnb	id	type	idsymbol	actual	nominal	uppertol	lowertol	deviation	exceed	warningLimitCF	featureid	featuresigma	comment	link	linkmode	mmc	useruppertol	userlowertol	fftphi	fftphiunit	zoneroundnessangle	groupname	groupname2	datumAid	datumBid	datumCid	natuppertolid	natlowertolid	decimalplaces	featurePosX	featurePosY	featurePosZ	unit	group1	group2	group3	group4	group5	group6	group7	group8	group9	group10	");
        
        let i = hm.into_iter().fold(0usize,|acc, x| acc.max(x.1));

        
        println!("{}", i);

        // assert_eq!(hm.get("id").unwrap(), 2);
        // assert_eq!(hm.get("end").unwrap_or(&0), 0);
    }


    struct OrderedMap<'a>{
        map: &'a HType,
        keys: &'a Vec<String>,
        index: usize
    }

    impl<'a> OrderedMap<'a> {
        fn new(map: &'a HType, keys: &'a Vec<String>)->Self{
            OrderedMap { map, keys, index: 0 }

        }
    }

    impl<'a> Iterator for OrderedMap<'a> {
        type Item = (&'a String, &'a Value);
        fn next(&mut self) -> Option<Self::Item> {
            
            if self.index >= self.keys.len(){
                return None;
            }

            let key = &self.keys[self.index];
            let value = self.map.get(key).unwrap();
            self.index += 1;

            Some((key, value))
        }
    }

    impl<'a> serde::ser::Serialize for OrderedMap<'a> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where S: serde::Serializer {
                
                let mut s = serializer.serialize_map(Some(self.keys.len()))?;
                for key in self.keys {
                    let item = self.map.get(key).unwrap();

                    s.serialize_key(key)?;
                    s.serialize_value(item)?;
                }
                s.end()
    
        }
    }

    #[test]
    fn orderedMapTest() {
        let mut m = HType::new();

        m.insert("1".to_string(), json!(1.0));
        m.insert("2".to_string(), json!(2.0));
        m.insert("3".to_string(), json!(3.0));

        let v = vec!["3".to_string(), "2".to_string()];
        let o = OrderedMap::new(&m, &v);
        
        println!("{}", serde_json::to_string(&o).unwrap())

    }
}
