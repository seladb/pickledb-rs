use std::collections::HashMap;
use std::fmt;
use serde::{de::DeserializeOwned, Serialize};
use serde_json;
use bincode;
use serde_yaml;
use serde_cbor;

/// An enum for specifying the serialization method to use when creating a new PickleDB database
/// or loading one from a file 
#[derive(Debug)]
pub enum SerializationMethod {
    /// [JSON serialization](https://crates.io/crates/serde_json)
    Json,

    /// [Bincode serialization](https://crates.io/crates/bincode)
    Bin,

    /// [YAML serialization](https://crates.io/crates/serde_yaml)
    Yaml,

    /// [CBOR serialization](https://crates.io/crates/serde_cbor)
    Cbor
}

impl From<i32> for SerializationMethod {
    fn from(item: i32) -> Self {
        match item {
            0 => SerializationMethod::Json,
            1 => SerializationMethod::Bin,
            2 => SerializationMethod::Yaml,
            3 => SerializationMethod::Cbor,
            _ => SerializationMethod::Json
        }
    }
}

impl fmt::Display for SerializationMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
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


struct YamlSerializer { }

impl YamlSerializer {
    fn new() -> YamlSerializer {
        YamlSerializer {}
    }

    fn deserialize_data<V>(&self, ser_data: &[u8]) -> Option<V> 
        where 
            V: DeserializeOwned    
    {
        match serde_yaml::from_str(std::str::from_utf8(ser_data).unwrap()) {
            Ok(val) => Some(val),
            Err(_) => None
        }
    }

    fn serialize_data<V>(&self, data: &V) -> Result<Vec<u8>, String>
            where
                V: Serialize
    {
        match serde_yaml::to_string(data) {
            Ok(ser_data) => Ok(ser_data.into_bytes()),
            Err(err) => Err(err.to_string())
        }
    }

    fn serialize_db(&self, map: &HashMap<String, Vec<u8>>, list_map: &HashMap<String, Vec<Vec<u8>>>) -> Result<Vec<u8>, String>{
        let mut yaml_map: HashMap<&str, &str> = HashMap::new();
        for (key, value) in map.iter() {
            yaml_map.insert(key, std::str::from_utf8(value).unwrap());
        }

        let mut yaml_list_map: HashMap<&str, Vec<&str>> = HashMap::new();
        for (key, list) in list_map.iter() {
            let yaml_list: Vec<&str> = list.iter().map(|item| std::str::from_utf8(item).unwrap()).collect();
            yaml_list_map.insert(key, yaml_list);
        }

        match serde_yaml::to_string(&(yaml_map, yaml_list_map)) {
            Ok(ser_db) => Ok(ser_db.into_bytes()),
            Err(err) => Err(err.to_string())
        }
    }

    fn deserialize_db(&self, ser_db: &[u8]) -> Result<(HashMap<String, Vec<u8>>, HashMap<String, Vec<Vec<u8>>>), String> {
        match serde_yaml::from_str::<(HashMap<String, String>, HashMap<String, Vec<String>>)>(std::str::from_utf8(ser_db).unwrap()) {
            Ok((yaml_map, yaml_list_map)) => {
                let mut byte_map: HashMap<String, Vec<u8>> = HashMap::new();
                for (key, value) in yaml_map.iter() {
                    byte_map.insert(key.to_string(), value.as_bytes().to_vec());
                }

                let mut byte_list_map: HashMap<String, Vec<Vec<u8>>> = HashMap::new();
                for (key, list) in yaml_list_map.iter() {
                    let byte_list: Vec<Vec<u8>> = list.iter().map(|item| item.as_bytes().to_vec()).collect();
                    byte_list_map.insert(key.to_string(), byte_list);
                }

                Ok((byte_map, byte_list_map))
            },

            Err(err) => Err(err.to_string())
        }
    }
}


struct BincodeSerializer { }

impl BincodeSerializer {
    fn new() -> BincodeSerializer {
        BincodeSerializer {}
    }

    fn deserialize_data<V>(&self, ser_data: &[u8]) -> Option<V> 
        where 
            V: DeserializeOwned    
    {
        match bincode::deserialize(ser_data) {
            Ok(val) => Some(val),
            Err(_) => None
        }
    }

    fn serialize_data<V>(&self, data: &V) -> Result<Vec<u8>, String>
            where
                V: Serialize
    {
        match bincode::serialize(data) {
            Ok(ser_data) => Ok(ser_data),
            Err(err) => Err(err.to_string())
        }        
    }

    fn serialize_db(&self, map: &HashMap<String, Vec<u8>>, list_map: &HashMap<String, Vec<Vec<u8>>>) -> Result<Vec<u8>, String> {
        self.serialize_data(&(map, list_map))
    }

    fn deserialize_db(&self, ser_db: &[u8]) -> Result<(HashMap<String, Vec<u8>>, HashMap<String, Vec<Vec<u8>>>), String> {
        match self.deserialize_data(ser_db) {
            Some((map, list_map)) => Ok((map, list_map)),
            None => Err(String::from("Cannot deserialize DB"))
        }
    }
}


struct CborSerializer { }

impl CborSerializer {
    fn new() -> CborSerializer {
        CborSerializer {}
    }

    fn deserialize_data<V>(&self, ser_data: &[u8]) -> Option<V> 
        where 
            V: DeserializeOwned    
    {
        match serde_cbor::from_slice(ser_data) {
            Ok(val) => Some(val),
            Err(_) => None
        }
    }

    fn serialize_data<V>(&self, data: &V) -> Result<Vec<u8>, String>
            where
                V: Serialize
    {
        match serde_cbor::to_vec(data) {
            Ok(ser_data) => Ok(ser_data),
            Err(err) => Err(err.to_string())
        }        
    }

    fn serialize_db(&self, map: &HashMap<String, Vec<u8>>, list_map: &HashMap<String, Vec<Vec<u8>>>) -> Result<Vec<u8>, String> {
        self.serialize_data(&(map, list_map))
    }

    fn deserialize_db(&self, ser_db: &[u8]) -> Result<(HashMap<String, Vec<u8>>, HashMap<String, Vec<Vec<u8>>>), String> {
        match self.deserialize_data(ser_db) {
            Some((map, list_map)) => Ok((map, list_map)),
            None => Err(String::from("Cannot deserialize DB"))
        }
    }
}


pub(crate) struct Serializer {
    ser_method: SerializationMethod,
    json_serializer: JsonSerializer,
    bincode_serializer: BincodeSerializer,
    yaml_serializer: YamlSerializer,
    cbor_serializer: CborSerializer
}

impl Serializer {

    pub(crate) fn new(ser_method: SerializationMethod) -> Serializer {
        Serializer {
            ser_method: ser_method,
            json_serializer: JsonSerializer::new(),
            bincode_serializer: BincodeSerializer::new(),
            yaml_serializer: YamlSerializer::new(),
            cbor_serializer: CborSerializer::new(),
        }

    }

    pub(crate) fn deserialize_data<V>(&self, ser_data: &[u8]) -> Option<V> 
        where 
            V: DeserializeOwned    
    {
        match self.ser_method {
            SerializationMethod::Json => self.json_serializer.deserialize_data(ser_data),
            SerializationMethod::Bin => self.bincode_serializer.deserialize_data(ser_data),
            SerializationMethod::Yaml => self.yaml_serializer.deserialize_data(ser_data),
            SerializationMethod::Cbor => self.cbor_serializer.deserialize_data(ser_data)
        }
    }

    pub(crate) fn serialize_data<V>(&self, data: &V) -> Result<Vec<u8>, String>
            where
                V: Serialize
    {
        match self.ser_method {
            SerializationMethod::Json => self.json_serializer.serialize_data(data),
            SerializationMethod::Bin => self.bincode_serializer.serialize_data(data),
            SerializationMethod::Yaml => self.yaml_serializer.serialize_data(data),
            SerializationMethod::Cbor => self.cbor_serializer.serialize_data(data)
        }
    }

    pub(crate) fn serialize_db(&self, map: &HashMap<String, Vec<u8>>, list_map: &HashMap<String, Vec<Vec<u8>>>) -> Result<Vec<u8>, String> {
        match self.ser_method {
            SerializationMethod::Json => self.json_serializer.serialize_db(map, list_map),
            SerializationMethod::Bin => self.bincode_serializer.serialize_db(map, list_map),
            SerializationMethod::Yaml => self.yaml_serializer.serialize_db(map, list_map),
            SerializationMethod::Cbor => self.cbor_serializer.serialize_db(map, list_map)
        }
    }

    pub(crate) fn deserialize_db(&self, ser_db: &[u8]) -> Result<(HashMap<String, Vec<u8>>, HashMap<String, Vec<Vec<u8>>>), String> {
        match self.ser_method {
            SerializationMethod::Json => self.json_serializer.deserialize_db(ser_db),
            SerializationMethod::Bin => self.bincode_serializer.deserialize_db(ser_db),
            SerializationMethod::Yaml => self.yaml_serializer.deserialize_db(ser_db),
            SerializationMethod::Cbor => self.cbor_serializer.deserialize_db(ser_db)
        }
    }
}

