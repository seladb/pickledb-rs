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
        match serde_json::to_string(&(map, list_map)) {
            Ok(ser_db) => Ok(ser_db.into_bytes()),
            Err(err) => Err(err.to_string())
        }
    }

    fn deserialize_db(&self, ser_db: &[u8]) -> Result<(HashMap<String, Vec<u8>>, HashMap<String, Vec<Vec<u8>>>), String> {
        match serde_json::from_str(std::str::from_utf8(ser_db).unwrap()) {
            Ok(deser_db) => Ok(deser_db),
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

