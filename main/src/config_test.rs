
use serde::{Deserialize, Serialize};
use toml;


#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Machine{
    #[serde(default, alias = "type")]
    pub machine_type: String,
    pub dmesn: String,
    pub dmeswv: String,
    pub controllertyp: String,
    pub dmeswi: String,
    pub dmeid: String
} 

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Config{
    pub machine: Machine,
    pub export: Vec<toml::Value>
}


#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Xml {
    #[serde(alias="type")]
    export_type: String,
    xml_result_path: String
}

#[cfg(test)]
mod tests{
    use std::fs::read_to_string;
    use toml::{self, value::Map};
    use crate::config_test::{Config, Machine};

    #[test]
    fn test_read_toml() -> std::io::Result<()>{
        let t = read_to_string("../config.toml")?;
    
        let data: Config = toml::from_str(t.as_str())?;
        

        let mut m1:Map<String, toml::Value> = Map::new();
        m1.insert(String::from("type"), toml::Value::String(String::from("xml")));
        m1.insert(String::from("xml_result_path"), toml::Value::String(String::from("O:\\Measurement\\results")));

        let mut m2:Map<String, toml::Value> = Map::new();
        m2.insert(String::from("type"), toml::Value::String(String::from("cmm")));
        m2.insert(String::from("cmm_result_path"), toml::Value::String(String::from(".\\test_data")));

        let mut m3:Map<String, toml::Value> = Map::new();
        m3.insert(String::from("type"), toml::Value::String(String::from("mongodb")));
        m3.insert(String::from("uri"), toml::Value::String(String::from("mongodb:\\urlToMongodb")));
        m3.insert(String::from("database"), toml::Value::String(String::from("cmmData")));
        m3.insert(String::from("collection"), toml::Value::String(String::from("results")));


        let config = Config{
            machine: Machine { 
                machine_type: String::from("zeiss"), 
                dmesn: String::from("501717"), 
                dmeswv: String::from("6.8.2606"),
                controllertyp: String::from("C32BIT"),
                dmeswi: String::from("Calypso"),
                dmeid: String::from("CONT_G2") 
            },
            export: vec![toml::Value::Table(m1), toml::Value::Table(m2), toml::Value::Table(m3)]
        };

        
        assert_eq!(data, config);
        Ok(())
    }
    
}
