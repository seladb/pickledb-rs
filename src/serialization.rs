use std::collections::HashMap;
use serde::{de::DeserializeOwned, Serialize};
use serde_json;

pub enum SerializationMethod {
    Json
}

struct JsonSerializer { }

impl JsonSerializer {
    fn new() -> JsonSerializer {
        JsonSerializer {}
    }

    fn deserialize_data<V>(&self, ser_data: &[u8]) -> Option<V> 
        where 
            V: DeserializeOwned    
    {
        match serde_json::from_str(std::str::from_utf8(ser_data).unwrap()) {
            Ok(val) => Some(val),
            Err(_) => None
        }
    }

    fn serialize_data<V>(&self, data: &V) -> Result<Vec<u8>, String>
            where
                V: Serialize
    {
        match serde_json::to_string(data) {
            Ok(ser_data) => Ok(ser_data.into_bytes()),
            Err(err) => Err(err.to_string())
        }
    }

    fn serialize_db(&self, map: &HashMap<String, Vec<u8>>, list_map: &HashMap<String, Vec<Vec<u8>>>) -> Result<Vec<u8>, String>{
        let mut json_map: HashMap<&str, &str> = HashMap::new();
        for (key, value) in map.iter() {
            json_map.insert(key, std::str::from_utf8(value).unwrap());
        }

        let mut json_list_map: HashMap<&str, Vec<&str>> = HashMap::new();
        for (key, list) in list_map.iter() {
            let json_list: Vec<&str> = list.iter().map(|item| std::str::from_utf8(item).unwrap()).collect();
            json_list_map.insert(key, json_list);
        }

        match serde_json::to_string(&(json_map, json_list_map)) {
            Ok(ser_db) => Ok(ser_db.into_bytes()),
            Err(err) => Err(err.to_string())
        }
    }

    fn deserialize_db(&self, ser_db: &[u8]) -> Result<(HashMap<String, Vec<u8>>, HashMap<String, Vec<Vec<u8>>>), String> {
        match serde_json::from_str::<(HashMap<String, String>, HashMap<String, Vec<String>>)>(std::str::from_utf8(ser_db).unwrap()) {
            Ok((json_map, json_list_map)) => {
                let mut byte_map: HashMap<String, Vec<u8>> = HashMap::new();
                for (key, value) in json_map.iter() {
                    byte_map.insert(key.to_string(), value.as_bytes().to_vec());
                }

                let mut byte_list_map: HashMap<String, Vec<Vec<u8>>> = HashMap::new();
                for (key, list) in json_list_map.iter() {
                    let byte_list: Vec<Vec<u8>> = list.iter().map(|item| item.as_bytes().to_vec()).collect();
                    byte_list_map.insert(key.to_string(), byte_list);
                }

                Ok((byte_map, byte_list_map))
            },

            Err(err) => Err(err.to_string())
        }
    }
}

pub(crate) struct Serializer {
    ser_method: SerializationMethod,
    json_serializer: JsonSerializer
}

impl Serializer {

    pub(crate) fn new(ser_method: SerializationMethod) -> Serializer {
        Serializer {
            ser_method: ser_method,
            json_serializer: JsonSerializer::new()
        }

    }

    pub(crate) fn deserialize_data<V>(&self, ser_data: &[u8]) -> Option<V> 
        where 
            V: DeserializeOwned    
    {
        match self.ser_method {
            SerializationMethod::Json => self.json_serializer.deserialize_data(ser_data)
        }
    }

    pub(crate) fn serialize_data<V>(&self, data: &V) -> Result<Vec<u8>, String>
            where
                V: Serialize
    {
        match self.ser_method {
            SerializationMethod::Json => self.json_serializer.serialize_data(data)
        }
    }

    pub(crate) fn serialize_db(&self, map: &HashMap<String, Vec<u8>>, list_map: &HashMap<String, Vec<Vec<u8>>>) -> Result<Vec<u8>, String> {
        match self.ser_method {
            SerializationMethod::Json => self.json_serializer.serialize_db(map, list_map)
        }
    }

    pub(crate) fn deserialize_db(&self, ser_db: &[u8]) -> Result<(HashMap<String, Vec<u8>>, HashMap<String, Vec<Vec<u8>>>), String> {
        match self.ser_method {
            SerializationMethod::Json => self.json_serializer.deserialize_db(ser_db)
        }
    }
}

