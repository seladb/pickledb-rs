use serde::{de::DeserializeOwned, Serialize};
use serde_json;

pub(crate) fn deserialize_data<V>(ser_data: &str) -> Option<V> 
    where 
        V: DeserializeOwned    
{
    match serde_json::from_str(&ser_data) {
        Ok(val) => Some(val),
        Err(_) => None
    }
}

pub(crate) fn serialize_data<V>(data: &V) -> Result<String, String>
        where
            V: Serialize
{
    match serde_json::to_string(data) {
        Ok(ser_data) => Ok(ser_data),
        Err(err) => Err(err.to_string())
    }
}