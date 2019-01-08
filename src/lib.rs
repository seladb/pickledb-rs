use std::io::Error;
use std::collections::HashMap;
use std::fs;
use serde::{de::DeserializeOwned, Serialize};
use serde_json;

pub struct PickleDb {
    map: HashMap<String, String>, 
    list_map: HashMap<String, Vec<String>>,
    db_file_path: String,
    auto_dump: bool,
}

impl PickleDb {
    pub fn new(location: &str, auto_dump: bool) -> PickleDb {
        PickleDb { map: HashMap::new(), list_map: HashMap::new(), db_file_path: String::from(location), auto_dump: auto_dump }
    }

    pub fn load(location: &str, auto_dump: bool) -> Result<PickleDb, Error> {
        let contents = fs::read_to_string(location)?;
        let map_from_file: (_,_) = serde_json::from_str(&contents)?;
        Ok(PickleDb { map: map_from_file.0, list_map: map_from_file.1, db_file_path: String::from(location), auto_dump: auto_dump })
    }

    pub fn dump(&self) -> bool {
        match serde_json::to_string(&(&self.map, &self.list_map)) {
            Ok(db_as_json) => {
                fs::write(&self.db_file_path, &db_as_json).expect("Unable to write file");
                true
            }
            Err(_) => false,
        }
    }

    fn dumpdb(&self) {
        if self.auto_dump {
            self.dump();
        }
    }

    pub fn set<V>(&mut self, key: &str, value: &V)
        where
            V: Serialize
    {
        self.map.insert(String::from(key), serde_json::to_string(value).unwrap());
        self.dumpdb();
    }

    pub fn get<V>(&self, key: &str) -> Option<V> 
        where 
            V: DeserializeOwned
    {
        match self.map.get(key) {
            Some(val_as_string) => match serde_json::from_str(&val_as_string) {
                Ok(val) => Some(val),
                Err(_) => None
            },
            
            None => None,
        }
    }

    pub fn exists(&self, key: &str) -> bool {
        self.map.get(key).is_some() || self.list_map.get(key).is_some()
    }

    pub fn get_as_json(&self, key: &str) -> Option<&String> {
        self.map.get(&String::from(key))
    }

    pub fn get_all(&self) -> Vec<String> {
        [self.map
            .iter()
            .map(|(key, _)| key.clone())
            .collect::<Vec<String>>(),

        self.list_map
            .iter()
            .map(|(key, _)| key.clone())
            .collect::<Vec<String>>()]
        
        .concat()
    }

    pub fn total_keys(&self) -> usize {
        self.map.iter().len() + self.list_map.iter().len()
    }

    pub fn rem(&mut self, key: &str) -> bool {
        let res = self.map.remove(key).is_some() || self.list_map.remove(key).is_some();
        self.dumpdb();
        res
    }

    pub fn lcreate(&mut self, name: &str) {
        let new_list: Vec<String> = Vec::new();
        self.list_map.insert(String::from(name), new_list);
        self.dumpdb();
    }

    pub fn lexists(&self, name: &str) -> bool {
        self.list_map.get(name).is_some()
    }

    pub fn ladd<V>(&mut self, name: &str, value: &V) -> bool
        where
            V: Serialize
    {
        self.lextend(name, &vec![value])
    }

    pub fn lextend<V>(&mut self, name: &str, seq: &Vec<V>) -> bool
        where
            V: Serialize
    {
        match self.list_map.get_mut(name) {
            Some(list) => {
                let serialized: Vec<String> = seq.iter()
                .map(|x| serde_json::to_string(x).unwrap())
                .collect();
                list.extend(serialized);
                self.dumpdb();
                true
            },

            None => false,
        }
    }

    pub fn lget<V>(&self, name: &str, pos: usize) -> Option<V>
        where
            V: DeserializeOwned
    {
        match self.list_map.get(name) {
            Some(list) => match list.get(pos) {
                Some(val_as_string) => match serde_json::from_str(&val_as_string) {
                    Ok(val) => Some(val),
                    Err(_) => None,
                }
                None => None,
            }
            None => None,
        }
    }

    pub fn llen(&self, name: &str) -> usize {
        match self.list_map.get(name) {
            Some(list) => list.len(),
            None => 0
        }
    }

    pub fn lrem_list(&mut self, name: &str) -> usize {
        let res = self.llen(name);
        self.list_map.remove(name);
        self.dumpdb();
        res
    }

    pub fn lpop<V>(&mut self, name: &str, pos: usize) -> Option<V> 
        where
            V: DeserializeOwned
    {
        match self.list_map.get_mut(name) {
            Some(list) => {
                if pos < list.len() {
                    let res = list.remove(pos);
                    self.dumpdb();
                    match serde_json::from_str(&res) {
                        Ok(val) => Some(val),
                        Err(_) => None,
                    }
                } else {
                    None
                }
            },
                
            None => None,
        }
    }

    pub fn lrem_value<V>(&mut self, name: &str, value: &V) -> bool 
        where
            V: Serialize
    {
        match self.list_map.get_mut(name) {
            Some(list) => {
                let serialized_value = serde_json::to_string(&value).unwrap();
                match list.iter().position(|x| *x == serialized_value) {
                    Some(pos) => {
                        list.remove(pos);
                        self.dumpdb();
                        true
                    },

                    None => false,
                }
            },

            None => false,
        }
    }
}