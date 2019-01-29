//! PickleDB
//! ========
//! 
//! PickleDB-rs is a lightweight and simple key-value store written in Rust, heavily inspired by [Python's PickleDB](https://pythonhosted.org/pickleDB/)
//! 
//! PickleDB's architecture is very simple and straight-forward: the whole key-value data structure is stored in memory and is dumped to a file
//! periodically according to a policy defined by the user. There are APIs to create a new key-value store in memory or to load it from a file.
//! Everything runs in the user's process and thread and in its memory, which means that the key-value data will be stored in the user 
//! process's memory and each API call will access that key-value store directly and may trigger a dump to the DB file. There are no additional 
//! threads or processes created throughout the life-cycle of any of the APIs.
//! 
//! ## So what is it useful for? 
//! 
//! Basically for any use case that needs a simple and relatively small key-value store that can run in-process and
//! be stored in a file. Most of the key-value stores out there provide high scalability, performance and robustness, but in the cost of a very 
//! complex architecure, a lot of installation and configuration, and in many cases require a descent amount of resources. 
//! But sometimes you don't need this scalability and performance and all you need is a simple solution that can be easily set up and is easy to
//! use and understand. That's where PickleDB-rs comes into picture! I personally encountered several use cases like that and that's how I came 
//! to know about [Python's PickleDB](https://pythonhosted.org/pickleDB/), and I thought it'd be nice to build one in Rust as well.
//! 
//! ## Main features
//! 
//! Like the [Python's PickleDB](https://pythonhosted.org/pickleDB/), the API is very much inspired by Redis API and provides the following
//! main capabilities:
//! * Create a new key-value store in memory or load it from a file
//! * Dump the key-value store to a file according to a user-defined policy
//! * Set and get key-value pairs. A very unique feature in PickleDB is that the key-value map is heterogeneous. Please see more details below
//! * Manage lists. Every list has a name (which is its key in the key-value store) and a list of items it stores. PickleDB provides APIs to 
//!   create and delete lists and to add or remove items from them. Lists are also heterogeneous, meaning each list can store objects of different 
//!   types. Please see more details below
//! * Iterate over keys and values in the DB and over items in a list
//! 
//! Please take a look at the API documentation to get more details.
//! 
//! ## PickleDB provides heterogeneous map and lists!
//! 
//! Heterogeneous data structures are the ones in which the data elements doesn't belong to the same data type. All the data elements have 
//! different data types. As you know, Rust doesn't have a built-it mechanism for working with heterogeneous data structures. For example: it's not 
//! easy to define a list where each element has a different data type, and it's also not easy to define a map which contains keys or values of different 
//! data types. PickleDB tries to address this challenge and allows values to be of any type and also build lists that contains items of different data
//! types. It achieves that using serialization, which you can read more about below. This is a pretty cool feature that you may find very useful. 
//! The different types that are supported are:
//! * All primitive types
//! * Strings
//! * Vectors
//! * Tuples
//! * Strcuts and Enums that are serializable (please read more below)
//! 
//! ## Serialization
//! 
//! Serialization is an important part of PickleDB. It is the way heterogeneous data structures are enabled: instead of saving the actual object,
//! PickleDB stores a serialized version of it. That way all objects are "normalized" to the same type and can be stored in Rust data structures
//! such as a HashMap or a Vector.
//! 
//! Serialization is also the way data is stored in a file: before saving to the file, all data in memory is serialized and then it is written to
//! the file; upon loading the serialized data is read from the file and then deserialized to memory. Of course serialization and deserialization has 
//! their performance cost but high performance is not one of PickleDB's main objectives and I think it's a fair price to pay for achieving 
//! heterogeneous data structures.
//! 
//! In order to achieve this magic, all objects must be serializable. PickleDB uses the [Serde](https://serde.rs/) library for serialization and 
//! it currently supports only [JSON serialization](https://docs.serde.rs/serde_json/). In the future I intend to add more serialization options
//! such as [bincode](https://crates.io/crates/bincode) or [pickle](https://crates.io/crates/serde-pickle).
//! 
//! So what does it mean that all objects must be serializable? That means that all map values and list items that you use must be serializable.
//! Fortunately Serde already provides out-of-the-box serialization for most of the common objects: all primitive types, strings, vectors and tuples
//! are already serializable and you don't need to do anything to use them. But if you want to define your own structs or enums, you need to make sure 
//! they're serializable, which means that:
//! * They should include the  `#[derive(Serialize, Deserialize)]` macro. Please see [here](https://serde.rs/derive.html) for more details
//! * If a struct contains non-primitive members, they should be serializable as well
//! * You should include `serde = "1.0"` and `serde_derive = "1.0"` dependencies in your `Cargo.toml` file
//! 
//! You can take a look at the examples provided with PickleDB to get a better idea of how this works. 
//! 
//! ## Dumping data to a file
//! 
//! As mentioned before, PickleDB stores all the data in a file for persistency. Dumping data to a file is pretty expensive in terms of time and
//! performance, for various reasons:
//! * Everything in PickleDB runs in the user process context (including file writes), so frequent writes will affect the user process's performance
//! * The current implementation dumps all of the data into the file, which gets more significant as data gets bigger
//! * Before writing to the file the data is being serialized, which also has a performance cost
//! 
//! Although performance is not a big concern for PickleDB, I felt it'd make sense to implement different dump policies for the user to choose when
//! creating a new DB or loading one from a file. Here are the different policies and the differences between them:
//! * [PickleDbDumpPolicy::NeverDump](enum.PickleDbDumpPolicy.html#variant.NeverDump) - never dump any change, file will always remain read-only. 
//!   When choosing this policy even calling to [dump()](struct.PickleDb.html#method.dump) won't dump the data.
//! * [PickleDbDumpPolicy::AutoDump](enum.PickleDbDumpPolicy.html#variant.AutoDump) - every change will be dumped immediately and automatically to the file
//! * [PickleDbDumpPolicy::DumpUponRequest](enum.PickleDbDumpPolicy.html#variant.DumpUponRequest) - data won't be dumped unless the user calls 
//!   [dump()](struct.PickleDb.html#method.dump) proactively to dump the data
//! * [PickleDbDumpPolicy::PeriodicDump(Duration)](enum.PickleDbDumpPolicy.html#variant.PeriodicDump) - changes will be dumped to the file periodically, 
//!   no sooner than the Duration provided by the user. The way this mechanism works is as follows: each time there is a DB change the last DB dump time 
//!   is checked. If the time that has passed since the last dump is higher than Duration, changes will be dumped, otherwise changes will not be dumped.  
//! 
//! Apart from this dump policy, persistency is also kept by a implementing the `Drop` trait for the `PickleDB` object which ensures all in-memory data 
//! is dumped to the file upon destruction of the object.
//! 
use std::io::Error;
use std::collections::HashMap;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use std::fs;
use serde::{de::DeserializeOwned, Serialize};

use crate::serialization::Serializer;

pub use crate::serialization::SerializationMethod;
pub use self::iterators::{PickleDbIterator, PickleDbIteratorItem, PickleDbListIterator, PickleDbListIteratorItem};

mod iterators;
mod serialization;

/// An enum that determines the policy of dumping PickleDB changes into the file 
pub enum PickleDbDumpPolicy {
    /// Never dump any change, file will always remain read-only
    NeverDump,
    /// Every change will be dumped immediately and automatically to the file
    AutoDump,
    /// Data won't be dumped unless the user calls [PickleDB::dump()](struct.PickleDb.html#method.dump) proactively to dump the data
    DumpUponRequest,
    /// Changes will be dumped to the file periodically, no sooner than the Duration provided by the user. 
    /// The way this mechanism works is as follows: each time there is a DB change the last DB dump time is checked. 
    /// If the time that has passed since the last dump is higher than Duration, changes will be dumped, 
    /// otherwise changes will not be dumped
    PeriodicDump(Duration),
}

/// A struct for extending PickleDB lists and adding more items to them
pub struct PickleDbListExtender<'a> {
    db: &'a mut PickleDb,
    list_name: String
}

impl<'a> PickleDbListExtender<'a> {
    /// Add a single item to an existing list.
    /// 
    /// As mentioned before, the lists are heterogeneous, meaning a single list can contain 
    /// items of different types. That means that the item can be of any type that is serializable.
    /// That includes all primitive types, vectors, tuples and every struct that has the 
    /// `#[derive(Serialize, Deserialize)` attribute.
    /// The method returns another `PickleDbListExtender` object that enables to continue adding
    /// items to the list.
    /// 
    /// # Arguments
    /// 
    /// * `value` - a reference of the item to add to the list
    /// 
    /// # Examples
    /// 
    /// ```rust,ignore
    /// // create a new list
    /// db.lcreate("list1")
    /// 
    /// // add items of different types to the list
    ///   .ladd(&100)
    ///   .ladd(&String::from("my string"))
    ///   .ladd(&vec!["aa", "bb", "cc"]);
    /// ```
    /// 
    pub fn ladd<V>(&mut self, value: &V) -> PickleDbListExtender
        where
            V: Serialize
    {
        self.db.ladd(&self.list_name, value).unwrap()
    }

    /// Add multiple items to an existing list.
    /// 
    /// As mentioned before, the lists are heterogeneous, meaning a single list can contain 
    /// items of different types. That means that the item can be of any type that is serializable.
    /// That includes all primitive types, vectors, tuples and every struct that has the 
    /// `#[derive(Serialize, Deserialize)` attribute.
    /// This method adds multiple items to the list, but since they're in a vector that means all
    /// of them are of the same type. Of course it doesn't mean that the list cannot contain items
    /// of other types as well, as you can see in the example below.
    /// The method returns another `PickleDbListExtender` object that enables to continue adding 
    /// items to the list.
    /// 
    /// # Arguments
    /// 
    /// * `seq` - a vector containing the new items to add to the list
    /// 
    /// # Examples
    /// 
    /// ```rust,ignore
    /// // create a new list
    /// db.lcreate("list1");
    /// 
    /// // add a bunch of numbers to the list
    /// db.lextends("list1", &vec![100, 200, 300])
    /// 
    /// // add a bunch of strings to the list
    ///   .lextends(&vec!["aa", "bb", "cc"]);
    /// 
    /// // now the list contains 6 items and looks like this: [100, 200, 300, "aa, "bb", "cc"]
    /// ```
    /// 
    pub fn lextend<V>(&mut self, seq: &Vec<V>) -> PickleDbListExtender
        where
            V: Serialize
    {
        self.db.lextend(&self.list_name, seq).unwrap()
    }
}

/// A struct that represents a PickleDB object
pub struct PickleDb {
    map: HashMap<String, Vec<u8>>, 
    list_map: HashMap<String, Vec<Vec<u8>>>,
    serializer: Serializer,
    db_file_path: String,
    dump_policy: PickleDbDumpPolicy,
    last_dump: Instant
}

impl PickleDb {

    /// Constructs a new `PickleDB` instance.
    /// 
    /// # Arguments
    /// 
    /// * `location` - a path where the DB will be stored
    /// * `dump_policy` - an enum value that determines the policy of dumping DB changes into the file. Please see
    ///    [PickleDB::load()](#method.load) to understand the different policy options
    /// 
    /// # Examples
    /// 
    /// ```rust,ignore
    /// use pickledb::PickleDb;
    /// 
    /// let mut db = PickleDB::new("example.db", false);
    /// ```
    /// 
    /// TODO: fix doc
    pub fn new_json(location: &str, dump_policy: PickleDbDumpPolicy) -> PickleDb {
        PickleDb::new(location, dump_policy, SerializationMethod::Json)
    }

    /// TODO: fix doc
    pub fn new_bin(location: &str, dump_policy: PickleDbDumpPolicy) -> PickleDb {
        PickleDb::new(location, dump_policy, SerializationMethod::Bin)
    }

    /// TODO: fix doc
    pub fn new(location: &str, dump_policy: PickleDbDumpPolicy, serialization_method: SerializationMethod) -> PickleDb {
        PickleDb { 
            map: HashMap::new(), 
            list_map: HashMap::new(),
            serializer: Serializer::new(serialization_method),
            db_file_path: String::from(location), 
            dump_policy: dump_policy,
            last_dump: Instant::now() }
    }

    /// Load a DB from a file.
    /// 
    /// This method tries to load a DB from a file. Upon success an instance of `PickleDB` is returned, 
    /// otherwise an error is returned.
    /// 
    /// # Arguments
    /// 
    /// * `location` - a path where the DB is loaded from
    /// * `dump_policy` - an enum value that determines the policy of dumping DB changes into the file. 
    ///   The user can choose between the following options:
    ///   * [PickleDbDumpPolicy::NeverDump](enum.PickleDbDumpPolicy.html#variant.NeverDump) - never dump any change,
    ///     file will always remain read-only. When choosing this policy even calling to [dump()](#method.dump) won't dump the data.
    ///     Choosing this option is the same like calling [PickleDB::load_read_only()](#method.load_read_only)
    ///   * [PickleDbDumpPolicy::AutoDump](enum.PickleDbDumpPolicy.html#variant.AutoDump) - every change will be dumped
    ///     immediately and automatically to the file
    ///   * [PickleDbDumpPolicy::DumpUponRequest](enum.PickleDbDumpPolicy.html#variant.DumpUponRequest) - data won't be dumped
    ///     unless the user calls [dump()](#method.dump) proactively to dump the data
    ///   * [PickleDbDumpPolicy::PeriodicDump(Duration)](enum.PickleDbDumpPolicy.html#variant.PeriodicDump) - changes will be
    ///     dumped to the file periodically, no sooner than the Duration provided by the user. The way this mechanism works is
    ///     as follows: each time there is a DB change the last DB dump time is checked. If the time that has passed
    ///     since the last dump is higher than Duration, changes will be dumped, otherwise changes will not be dumped.    
    /// 
    /// # Examples
    /// 
    /// ```rust,ignore
    /// use pickledb::PickleDb;
    /// 
    /// let db = PickleDB::load("example.db", PickleDbDumpPolicy::AutoDump);
    /// ```
    /// 
    /// TODO: fix doc
    pub fn load(location: &str, dump_policy: PickleDbDumpPolicy, serialization_method: SerializationMethod) -> Result<PickleDb, Error> {
        let content = fs::read(location)?;
        let serializer = Serializer::new(serialization_method);
        let maps_from_file: (_,_) = serializer.deserialize_db(&content).unwrap();
        Ok(PickleDb { 
            map: maps_from_file.0, 
            list_map: maps_from_file.1, 
            serializer: serializer,
            db_file_path: String::from(location), 
            dump_policy: dump_policy,
            last_dump: Instant::now()
            })
    }

    /// TODO: fix doc
    pub fn load_json(location: &str, dump_policy: PickleDbDumpPolicy) -> Result<PickleDb, Error> {
        PickleDb::load(location, dump_policy, SerializationMethod::Json)
    }

    /// TODO: fix doc
    pub fn load_bin(location: &str, dump_policy: PickleDbDumpPolicy) -> Result<PickleDb, Error> {
        PickleDb::load(location, dump_policy, SerializationMethod::Bin)
    }

    /// Load a DB from a file in read-only mode.
    ///
    /// This method is similar to the [PickleDB::load()](#method.load) method with the only difference
    /// that the file is loaded from DB with a dump policy of 
    /// [PickleDbDumpPolicy::NeverDump](enum.PickleDbDumpPolicy.html#variant.NeverDump), meaning
    /// changes will not be saved to the file, even when calling [dump()](#method.dump)
    /// 
    /// # Arguments
    /// 
    /// * `location` - a path where the DB is loaded from
    /// 
    /// # Examples
    /// 
    /// ```rust,ignore
    /// use pickledb::PickleDb;
    /// 
    /// let readonly_db = PickleDB::load("example.db");
    /// 
    /// // nothing happens by calling this method
    /// readonly_db.dump();
    /// ```
    /// 
    /// 
    /// TODO: fix doc
    pub fn load_read_only(location: &str, serialization_method: SerializationMethod) -> Result<PickleDb, Error> {
        PickleDb::load(location, PickleDbDumpPolicy::NeverDump, serialization_method)
    }

    /// Dump the data to the file.
    /// 
    /// Calling this method is necessary only if the DB is loaded or created with `auto_dump = true`.
    /// Otherwise the data is dumped to the file upon every change. This method returns `true` if
    /// dump is successful, false otherwise.
    /// 
    pub fn dump(&mut self) -> bool {
        if let PickleDbDumpPolicy::NeverDump = self.dump_policy {
            return true
        }

        match self.serializer.serialize_db(&self.map, &self.list_map) {
            Ok(ser_db) => {
                let temp_file_path = format!("{}.temp.{}", 
                    self.db_file_path, 
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs());
                fs::write(&temp_file_path, ser_db).expect("Unable to write to temp file");
                fs::rename(temp_file_path, &self.db_file_path).expect("Unable to rename file");
                if let PickleDbDumpPolicy::PeriodicDump(_dur) = self.dump_policy {
                    self.last_dump = Instant::now();
                }
                true
            }
            Err(_) => false,
        }
    }

    fn dumpdb(&mut self) {
        match self.dump_policy {
            PickleDbDumpPolicy::AutoDump => {
                self.dump();
            },
            PickleDbDumpPolicy::PeriodicDump(duration) => {
                let now = Instant::now();
                if now.duration_since(self.last_dump) > duration {
                    self.last_dump = Instant::now();
                    self.dump();
                }
            },

            _ => (),
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
    /// * `value` - a value of any serializable type
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
        self.map.insert(String::from(key), self.serializer.serialize_data(value).unwrap());
        self.dumpdb();
    }

    /// Get a value of a key.
    /// 
    /// The key is always a string but the value can be of any type. It's the user's
    /// responsibility to know the value type and give it while calling this method.
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
            Some(val) => self.serializer.deserialize_data::<V>(&val),
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
    /// For adding elements to the list please call [ladd()](#method.ladd) or [lextend()](#method.lextend).
    /// If another list or value is already set under this key, they will be overridden,
    /// meaning the new list will override the old list or value.
    /// The method returns an object of type `PickleDbListExtender` that enables to add items 
    /// to the newly created list
    /// 
    /// # Arguments
    /// 
    /// * `name` - the key of the list that will be created
    /// 
    pub fn lcreate(&mut self, name: &str) -> PickleDbListExtender {
        let new_list: Vec<Vec<u8>> = Vec::new();
        if self.map.contains_key(name) {
            self.map.remove(name);
        }
        self.list_map.insert(String::from(name), new_list);
        self.dumpdb();
        PickleDbListExtender { db: self, list_name: String::from(name) }
    }

    /// Check if a list exists.
    /// 
    /// This method returns `true` if the list name exists and `false` otherwise.
    /// The difference between this method and [exists()](#method.exists) is that this methods checks only
    /// for lists with that name (key) and [exists()](#method.exists) checks for both values and lists.
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
    /// The method returns a `Some(PickleDbListExtender)` object that enables to add more items to the list
    /// if the item was added successfully or `None` if the list name isn't found in the DB.
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
    /// // add items of different types to the list
    /// db.ladd("list1", &100).unwrap()
    ///   .ladd(&String::from("my string"))
    ///   .ladd(&vec!["aa", "bb", "cc"]);
    /// ```
    /// 
    pub fn ladd<V>(&mut self, name: &str, value: &V) -> Option<PickleDbListExtender>
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
    /// This method adds multiple items to the list, but since they're in a vector that means all
    /// of them are of the same type. Of course it doesn't mean that the list cannot contain items
    /// of other types as well, as you can see in the example below.
    /// The method return `Some(PickleDbListExtender)` that enables to add more items to the list 
    /// if all items were added successfully or `None` if the list name isn't found in the DB.
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
    /// db.lextends("list1", &vec![100, 200, 300])
    /// 
    /// // add a String item to the list
    ///   .ladd(&String::from("my string"))
    /// 
    /// // add a vector item to the list
    ///   .ladd(&vec!["aa", "bb", "cc"]);
    /// 
    /// // now the list contains 5 items and looks like this: [100, 200, 300, "my string", ["aa, "bb", "cc"]]
    /// ```
    /// 
    pub fn lextend<V>(&mut self, name: &str, seq: &Vec<V>) -> Option<PickleDbListExtender>
        where
            V: Serialize
    {
        let serializer = &self.serializer;
        match self.list_map.get_mut(name) {
            Some(list) => {
                let serialized: Vec<Vec<u8>> = seq.iter()
                .map(|x| serializer.serialize_data(x).unwrap())
                .collect();
                list.extend(serialized);
                self.dumpdb();
                Some(PickleDbListExtender { db: self, list_name: String::from(name)})
            },

            None => None,
        }
    }

    /// Get an item of of a certain list in a certain position.
    /// 
    /// This method takes a list name and a position inside the list 
    /// and retrieves the item in this position. It's the user's responsibility 
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
                Some(val) => self.serializer.deserialize_data::<V>(&val),
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
    /// This method is somewhat similar to [rem()](#method.rem) but with 2 small differences:
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
    /// item in this position and returns it to the user. It's the user's responsibility 
    /// to know what is the correct type of the item and give it while calling this method.
    /// Since the item in the lists are stored in a serialized way the returned object 
    /// is not a reference to the item stored in a DB but actually a new instance of it.
    /// If the list is not found in the DB or the given position is out of bounds
    /// no item will be removed and `None` will be returned. Otherwise the item will be
    /// removed and `Some(V)` will be returned.
    /// This method is very similar to [lrem_value()](#method.lrem_value), the only difference is that this 
    /// methods returns the value and [lrem_value()](#method.lrem_value) returns only an indication whether
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
    /// let item2 = db.lpop::<i32>("list1", 2);
    /// 
    /// // item2 contains 3 and the list now looks like this: [1, 2, 4]
    /// 
    /// // remove item in position 1
    /// let item1 = db.lpop::<i32>("list1", 1);
    /// 
    /// // item1 contains 2 and the list now looks like this: [1, 4]
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
                    self.serializer.deserialize_data(&res)
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
    /// This method is very similar to [lpop()](#method.lpop), the only difference is that this 
    /// methods returns an indication and [lpop()](#method.lpop) returns the actual item that was removed.
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
                let serialized_value = self.serializer.serialize_data(&value).unwrap();
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

    /// Return an iterator over the keys and values in the DB.
    /// 
    /// # Examples
    /// 
    /// ```rust,ignore
    /// // iterate over all keys and values in the db
    /// for kv in db.iter() {
    ///     match kv.get_key() {
    ///         "key1" => println!("Value of {} is: {}", kv.get_key(), kv.get_value::<String>().unwrap()),
    ///         "key2" => println!("Value of {} is: {}", kv.get_key(), kv.get_value::<String>().unwrap()),
    ///         "key3" => println!("Value of {} is: {:?}", kv.get_key(), kv.get_value::<Vec<i32>>().unwrap()),
    ///         "key4" => println!("Value of {} is: {}", kv.get_key(), kv.get_value::<Rectangle>().unwrap()),
    ///         _ => ()
    ///     }
    /// }
    /// ```
    /// 
    pub fn iter(&self) -> PickleDbIterator {
        PickleDbIterator { map_iter: self.map.iter(), serializer: &self.serializer }
    }

    /// Return an iterator over the items in certain list.
    /// 
    /// # Arguments
    /// 
    /// * `name` - the list name. If the list doesn't exist an exception is thrown
    /// 
    /// # Examples
    /// 
    /// ```rust,ignore
    /// // create a new list
    /// db.lcreate("list1")
    ///   .lextend(&vec![1,2,3,4]);
    /// 
    /// // iterate over the items in list1
    /// for item_iter in db.liter("list1") {
    ///     println!("Current item is: {}", item_iter.get_item::<i32>().unwrap());
    /// }
    /// ```
    /// 
    pub fn liter(&self, name: &str) -> PickleDbListIterator {
        match self.list_map.get(name) {
            Some(list) => PickleDbListIterator { list_iter: list.iter(), serializer: &self.serializer },
            None => panic!("List '{}' doesn't exist", name)
        }
    }
}

impl Drop for PickleDb {
    fn drop(&mut self) {
        if let PickleDbDumpPolicy::NeverDump = self.dump_policy {
        } else {
            self.dump();
        }
    }
}