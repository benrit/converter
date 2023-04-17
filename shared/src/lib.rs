pub mod drivers;
pub mod hash_map_wrapper;



#[cfg(test)]
mod tests{

    use super::*;
    use std::collections::HashMap;
    use hash_map_wrapper::OrderedMap;
    use serde_json::json;
    
    #[test]
    fn ordered_map_test() {
        let mut m = HashMap::new();
    
        m.insert("1".to_string(), json!(1.0));
        m.insert("2".to_string(), json!(2.0));
        m.insert("3".to_string(), json!(3.0));
    
        let v = vec!["3", "2"];
        let o = OrderedMap::new(&m, &v);
        
        assert_eq!(serde_json::to_string(&o).unwrap(), "{\"3\":3.0,\"2\":2.0}")
    
    }
    
}
