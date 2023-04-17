use std::collections::HashMap;
use serde;
use serde::ser::SerializeMap;


#[derive(Debug)]
pub struct OrderedMap<'a, T>
where T: serde::Serialize
{
    map: &'a HashMap<String, T>,
    keys: &'a Vec<&'a str>,
    index: usize
}

impl<'a, T> OrderedMap<'a, T>
where T: serde::Serialize
 {
    pub fn new(map: &'a HashMap<String, T>, keys: &'a Vec<&str>)->Self{
        OrderedMap { map, keys, index: 0 }

    }
}

impl<'a, T> Iterator for OrderedMap<'a, T> 
where T: serde::Serialize
{
    type Item = (&'a str, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        
        if self.index >= self.keys.len(){
            return None;
        }

        let key = self.keys[self.index].to_string();
        let value = self.map.get(&key).unwrap();
        self.index += 1;

        Some((self.keys[self.index], value))
    }
}


impl<'a, T> serde::ser::Serialize for OrderedMap<'a, T> 
where T: serde::Serialize
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer
         {
            
            let mut s = serializer.serialize_map(Some(self.keys.len()))?;
            for key in self.keys {
                let item: &T = self.map.get(&key.to_string()).unwrap();

                s.serialize_key(key)?;
                s.serialize_value(item)?;
            }
            s.end()

    }
}