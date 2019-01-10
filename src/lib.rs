//! Crate documentation placeholder
//! 
//! 
//! 
//! 
use std::io::Error;
use std::collections::HashMap;
use std::fs;
use serde::{de::DeserializeOwned, Serialize};
use serde_json;

/// A struct that represents a PickleDB object
pub struct PickleDb {
    map: HashMap<String, String>, 
    list_map: HashMap<String, Vec<String>>,
    db_file_path: String,
    auto_dump: bool,
}

impl PickleDb {

    /// Constructs a new `PickleDB`.
    /// 
    /// # Arguments
    /// 
    /// * `location` - a path where the DB will be stored
    /// * `auto_dump` - a boolean that indicates whether or not each change should automatically
    ///   be saved to the file. If the value is false, nothing will be saved automatically and the
    ///   user should actively call the `dump()` method
    /// 
    /// # Examples
    /// 
    /// ```rust,ignore
    /// use pickledb::PickleDb;
    /// 
    /// let mut db = PickleDB::new("example.db", false);
    /// ```
    pub fn new(location: &str, auto_dump: bool) -> PickleDb {
        PickleDb { map: HashMap::new(), list_map: HashMap::new(), db_file_path: String::from(location), auto_dump: auto_dump }
    }

    /// Load a DB from a file.
    /// 
    /// This method tries to load a DB from a file. Upon success an instance of `PickleDB` is returned, 
    /// otherwise an error is returned.
    /// 
    /// # Arguments
    /// 
    /// * `location` - a path where the DB will be stored
    /// * `auto_dump` - a boolean that indicates whether or not each change should automatically
    ///   be saved to the file. If the value is false, nothing will be saved automatically and the
    ///   user should actively call the `dump()` method
    /// 
    /// # Examples
    /// 
    /// ```rust,ignore
    /// use pickledb::PickleDb;
    /// 
    /// let db = PickleDB::load("example.db", true);
    /// ```
    pub fn load(location: &str, auto_dump: bool) -> Result<PickleDb, Error> {
        let contents = fs::read_to_string(location)?;
        let map_from_file: (_,_) = serde_json::from_str(&contents)?;
        Ok(PickleDb { map: map_from_file.0, list_map: map_from_file.1, db_file_path: String::from(location), auto_dump: auto_dump })
    }

    /// Dump the data to the file.
    /// 
    /// Calling this method is necessary only if the DB is loaded or created with `auto_dump = true`.
    /// Otherwise the data is dumped to the file upon every change. This method returns `true` if
    /// dump is successful, false otherwise.
    /// 
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

    /// Set a key-value pair.
    /// 
    /// The key has to be a string but the value can be of any type that is serializable.
    /// That includes all primitive types, vectors, tuples and every struct that has the 
    /// `#[derive(Serialize, Deserialize)` attribute.
    /// 
    /// # Arguments
    /// 
    /// * `key` - a string key
    /// * `value` - a value of any serialzable type
    /// 
    /// # Examples
    /// 
    /// ```rust,ignore
    /// // set a number
    /// db.set("key1", &100);
    /// 
    /// // set a floating point number
    /// db.set("key2", &1.234);
    /// 
    /// // set a String
    /// db.set("key3", &String::from("hello world"));
    /// 
    /// // set a Vec
    /// db.set("key4", &vec![1,2,3]);
    /// 
    /// // set a struct
    /// #[derive(Serialize, Deserialize)]
    /// struct Coor {
    ///     x: i32,
    ///     y: i32,
    /// }
    /// let mycoor = Coor { x: 1, y : 2 };
    /// db.set("key5", &mycoor);
    /// ```
    /// 
    pub fn set<V>(&mut self, key: &str, value: &V)
        where
            V: Serialize
    {
        if self.list_map.contains_key(key) {
            self.list_map.remove(key);
        }
        self.map.insert(String::from(key), serde_json::to_string(value).unwrap());
        self.dumpdb();
    }

    /// Get a value of a key.
    /// 
    /// The key is always a string but the value can be of any type. It's the user's
    /// responisibility to know the value type and give it while calling this method.
    /// If the key doesn't exist or if the type is wrong, `None` will be returned.
    /// Otherwise `Some(V)` will be returned.
    /// Since the values are stored in a serialized way the returned object is
    /// not a reference to the value stored in a DB but actually a new instance of it
    /// 
    /// # Arguments
    /// 
    /// * `key` - a string key
    /// 
    /// # Examples
    /// 
    /// ```rust,ignore
    /// // read a num
    /// let num = db.get::<i32>("key1").unwrap();
    /// 
    /// // read a floating point number
    /// let float_num = db.get::<f32>("key2").unwrap();
    /// 
    /// // read a String
    /// let my_str = db.get::<String>("key3").unwrap();
    /// 
    /// // read a Vec
    /// ley vec = db.get::<Vec<i32>>("key4").unwrap();
    /// 
    /// // read a struct
    /// let coor = db.get::<Coor>("key5").unwrap();
    /// ```
    /// 
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

    /// Check if a key exists.
    /// 
    /// This method returns `true` if the key exists and `false` otherwise.
    /// 
    /// # Arguments
    /// 
    /// * `key` - the key to check
    /// 
    pub fn exists(&self, key: &str) -> bool {
        self.map.get(key).is_some() || self.list_map.get(key).is_some()
    }

    /// Get a vector of all the keys in the DB.
    /// 
    /// The keys returned in the vector are not references to the actual key string
    /// objects but rather a clone of them.
    /// 
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

    /// Get the total number of keys in the DB.
    /// 
    pub fn total_keys(&self) -> usize {
        self.map.iter().len() + self.list_map.iter().len()
    }

    /// Remove a key-value pair or a list from the DB.
    /// 
    /// This methods returns `true` if the key was found in the DB or false if it wasn't found
    /// 
    /// # Arguments
    /// 
    /// * `key` - the key or list name to remove
    /// 
    pub fn rem(&mut self, key: &str) -> bool {
        let res = self.map.remove(key).is_some() || self.list_map.remove(key).is_some();
        self.dumpdb();
        res
    }

    /// Create a new list.
    /// 
    /// This method just creates a new list, it doesn't add any elements to it.
    /// For adding elements to the list please call `ladd()` or `lextend()`.
    /// If another list or value is already set under this key, they will be overriden,
    /// meaning the new list will override the old list or value.
    /// 
    /// # Arguments
    /// 
    /// * `name` - the key of the list that will be created
    /// 
    pub fn lcreate(&mut self, name: &str) {
        let new_list: Vec<String> = Vec::new();
        if self.map.contains_key(name) {
            self.map.remove(name);
        }
        self.list_map.insert(String::from(name), new_list);
        self.dumpdb();
    }

    /// Check if a list exists.
    /// 
    /// This method returns `true` if the list name exists and `false` otherwise.
    /// The difference between this method and `exists()` is that this methods checks only
    /// for lists with that name (key) and `exists()` checks for both values and lists.
    /// 
    /// # Arguments
    /// 
    /// * `name` - the list key to check
    /// 
    pub fn lexists(&self, name: &str) -> bool {
        self.list_map.get(name).is_some()
    }

    /// Add a single item to an existing list.
    /// 
    /// As mentioned before, the lists are heterogeneous, meaning a single list can contain 
    /// items of different types. That means that the item can be of any type that is serializable.
    /// That includes all primitive types, vectors, tuples and every struct that has the 
    /// `#[derive(Serialize, Deserialize)` attribute.
    /// The method return `true` if the item was added successfully or `false` if the list name 
    /// isn't found in the DB.
    /// 
    /// # Arguments
    /// 
    /// * `name` - the list key
    /// * `value` - a reference of the item to add to the list
    /// 
    /// # Examples
    /// 
    /// ```rust,ignore
    /// // create a new list
    /// db.lcreate("list1");
    /// 
    /// // add a number item to the list
    /// db.ladd("list1", &100);
    /// 
    /// // add a String item to the list
    /// db.ladd("list1", &String::from("my string"));
    /// 
    /// // add a vector item to the list
    /// db.ladd("list1", &vec!["aa", "bb", "cc"]);
    /// ```
    /// 
    pub fn ladd<V>(&mut self, name: &str, value: &V) -> bool
        where
            V: Serialize
    {
        self.lextend(name, &vec![value])
    }

    /// Add multiple items to an existing list.
    /// 
    /// As mentioned before, the lists are heterogeneous, meaning a single list can contain 
    /// items of different types. That means that the item can be of any type that is serializable.
    /// That includes all primitive types, vectors, tuples and every struct that has the 
    /// `#[derive(Serialize, Deserialize)` attribute.
    /// This method adds multiple items to the list, but since they're in a vecotr that means all
    /// of them are of the same type. Of course it doesn't mean that the list cannot contain items
    /// of other types as well, as you can see in the example below.
    /// The method return `true` if all items were added successfully or `false` if the list name 
    /// isn't found in the DB.
    /// 
    /// # Arguments
    /// 
    /// * `name` - the list key
    /// * `seq` - a vector containing the new items to add to the list
    /// 
    /// # Examples
    /// 
    /// ```rust,ignore
    /// // create a new list
    /// db.lcreate("list1");
    /// 
    /// // add a bunch of numbers to the list
    /// db.lextends("list1", &vec![100, 200, 300]);
    /// 
    /// // add a String item to the list
    /// db.ladd("list1", &String::from("my string"));
    /// 
    /// // add a vector item to the list
    /// db.ladd("list1", &vec!["aa", "bb", "cc"]);
    /// 
    /// // now the list contains 5 items and looks like this: [100, 200, 300, "my string", ["aa, "bb", "cc"]]
    /// ```
    /// 
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

    /// Get an item of of a certain list in a certain position.
    /// 
    /// This method takes a list name and a position inside the list 
    /// and retrives the item in this position. It's the user's responisibility 
    /// to know what is the correct type of the item and give it while calling this method.
    /// Since the item in the lists are stored in a serialized way the returned object 
    /// is not a reference to the item stored in a DB but actually a new instance of it.
    /// If the list is not found in the DB or the given position is out of bounds
    /// of the list `None` will be returned. Otherwise `Some(V)` will be returned.
    /// 
    /// # Arguments
    /// 
    /// * `name` - the list key
    /// * `pos` - the position of the item inside the list. Expected value is >= 0
    /// 
    /// # Examples
    /// ```rust,ignore
    /// // create a list
    /// db.lcreate("list1");
    /// 
    /// // add a number to list1
    /// db.ladd("list1", &100));
    /// 
    /// // add a string to list1
    /// db.ladd("list1", &String::from("my string"));
    /// 
    /// // read the first item in the list - int
    /// let x = db.lget::<i32>("list1", 0).unwrap();
    /// 
    /// // read the second item in the list - string
    /// let s = db.lget::<String>("list1", 1).unwrap();
    /// ```
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

    /// Get the length of a list.
    /// 
    /// If the list is empty or if it doesn't exist the value of 0 is returned.
    /// 
    /// # Arguments
    /// 
    /// * `name` - the list key
    /// 
    pub fn llen(&self, name: &str) -> usize {
        match self.list_map.get(name) {
            Some(list) => list.len(),
            None => 0
        }
    }

    /// Remove a list.
    /// 
    /// This method is somewhat similar to `rem()` but with 2 small differences:
    /// * This method only removes lists and not key-value pairs
    /// * The return value of this method is the number of items that were in 
    ///   the list that was removed. If the list doesn't exist a value of 0 is
    ///   returned
    /// 
    /// # Arguments
    /// 
    /// * `name` - the list key to remove
    /// 
    pub fn lrem_list(&mut self, name: &str) -> usize {
        let res = self.llen(name);
        self.list_map.remove(name);
        self.dumpdb();
        res
    }

    /// Pop an item out of a list.
    /// 
    /// This method takes a list name and a position inside the list, removes the
    /// item in this position and returns it to the user. It's the user's responisibility 
    /// to know what is the correct type of the item and give it while calling this method.
    /// Since the item in the lists are stored in a serialized way the returned object 
    /// is not a reference to the item stored in a DB but actually a new instance of it.
    /// If the list is not found in the DB or the given position is out of bounds
    /// no item will be removed and `None` will be returned. Otherwise the item will be
    /// removed and `Some(V)` will be returned.
    /// This method is very similar to `lrem_value()`, the only difference is that this 
    /// methods returns the value and `lrem_value()` returns only an inidication whether
    /// the item was removed or not.
    /// 
    /// # Arguments
    /// 
    /// * `name` - the list key
    /// * `pos` - the position of the item to remove
    /// 
    /// # Examples
    /// 
    /// ```rust,ignore
    /// // create a list
    /// db.lcreate("list1");
    /// 
    /// // add 4 items to the list
    /// db.lextend("list1", &vec![1,2,3,4]);
    /// 
    /// // remove item in position 2
    /// let item2 = db.pop::<i32>("list1", 2);
    /// 
    /// // item2 cotnains 3 and the list now looks like this: [1, 2, 4]
    /// 
    /// // remove item in position 1
    /// let item1 = db.pop::<i32>("list1", 1);
    /// 
    /// // item1 cotnains 2 and the list now looks like this: [1, 4]
    /// ```
    /// 
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

    /// Remove an item out of a list.
    /// 
    /// This method takes a list name and a position inside the list, removes the
    /// item in this position and returns an indication whether the item was removed or not.
    /// If the list is not found in the DB or the given position is out of bounds
    /// no item will be removed and `false` will be returned. Otherwise the item will be
    /// removed and `true` will be returned.
    /// This method is very similar to `pop()`, the only difference is that this 
    /// methods returns an indication and `pop()` returns the actual item that was removed.
    /// 
    /// # Arguments
    /// 
    /// * `name` - the list key
    /// * `pos` - the position of the item to remove
    /// 
    /// # Examples
    /// 
    /// ```rust,ignore
    /// // create a list
    /// db.lcreate("list1");
    /// 
    /// // add 4 items to the list
    /// db.lextend("list1", &vec![1,2,3,4]);
    /// 
    /// // remove item in position 2
    /// db.lrem_value("list1", 2);
    /// 
    /// // The list now looks like this: [1, 2, 4]
    /// 
    /// // remove item in position 1
    /// db.lrem_value("list1", 1);
    /// 
    /// // The list now looks like this: [1, 4]
    /// ```
    /// 
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