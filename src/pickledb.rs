use serde::{de::DeserializeOwned, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use crate::error::{Error, ErrorCode, Result};
use crate::extenders::PickleDbListExtender;
use crate::iterators::{PickleDbIterator, PickleDbListIterator};
use crate::serialization::SerializationMethod;
use crate::serialization::Serializer;

/// An enum that determines the policy of dumping PickleDb changes into the file
pub enum PickleDbDumpPolicy {
    /// Never dump any change, file will always remain read-only
    NeverDump,
    /// Every change will be dumped immediately and automatically to the file
    AutoDump,
    /// Data won't be dumped unless the user calls [PickleDb::dump()](struct.PickleDb.html#method.dump) proactively to dump the data
    DumpUponRequest,
    /// Changes will be dumped to the file periodically, no sooner than the Duration provided by the user.
    /// The way this mechanism works is as follows: each time there is a DB change the last DB dump time is checked.
    /// If the time that has passed since the last dump is higher than Duration, changes will be dumped,
    /// otherwise changes will not be dumped
    PeriodicDump(Duration),
}

/// A struct that represents a PickleDb object
pub struct PickleDb {
    map: HashMap<String, Vec<u8>>,
    list_map: HashMap<String, Vec<Vec<u8>>>,
    serializer: Serializer,
    db_file_path: PathBuf,
    dump_policy: PickleDbDumpPolicy,
    last_dump: Instant,
}

impl PickleDb {
    /// Constructs a new `PickleDb` instance.
    ///
    /// # Arguments
    ///
    /// * `db_path` - a path where the DB will be stored
    /// * `dump_policy` - an enum value that determines the policy of dumping DB changes into the file. Please see
    ///    [PickleDb::load()](#method.load) to understand the different policy options
    /// * `serialization_method` - the serialization method to use for storing the data to memory and file
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
    ///
    /// let mut db = PickleDb::new("example.db", PickleDbDumpPolicy::AutoDump, SerializationMethod::Json);
    /// ```
    ///
    pub fn new<P: AsRef<Path>>(
        db_path: P,
        dump_policy: PickleDbDumpPolicy,
        serialization_method: SerializationMethod,
    ) -> PickleDb {
        let mut db_path_buf = PathBuf::new();
        db_path_buf.push(db_path);

        PickleDb {
            map: HashMap::new(),
            list_map: HashMap::new(),
            serializer: Serializer::new(serialization_method),
            db_file_path: db_path_buf,
            dump_policy,
            last_dump: Instant::now(),
        }
    }

    /// Constructs a new `PickleDb` instance that uses [JSON serialization](https://crates.io/crates/serde_json) for storing the data.
    ///
    /// # Arguments
    ///
    /// * `db_path` - a path where the DB will be stored
    /// * `dump_policy` - an enum value that determines the policy of dumping DB changes into the file. Please see
    ///    [PickleDb::load()](#method.load) to understand the different policy options
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use pickledb::{PickleDb, PickleDbDumpPolicy};
    ///
    /// let mut db = PickleDb::new_json("example.db", PickleDbDumpPolicy::AutoDump);
    /// ```
    ///
    #[cfg(feature = "json")]
    pub fn new_json<P: AsRef<Path>>(db_path: P, dump_policy: PickleDbDumpPolicy) -> PickleDb {
        PickleDb::new(db_path, dump_policy, SerializationMethod::Json)
    }

    /// Constructs a new `PickleDb` instance that uses [Bincode serialization](https://crates.io/crates/bincode) for storing the data.
    ///
    /// # Arguments
    ///
    /// * `db_path` - a path where the DB will be stored
    /// * `dump_policy` - an enum value that determines the policy of dumping DB changes into the file. Please see
    ///    [PickleDb::load()](#method.load) to understand the different policy options
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use pickledb::{PickleDb, PickleDbDumpPolicy};
    ///
    /// let mut db = PickleDb::new_bin("example.db", PickleDbDumpPolicy::AutoDump);
    /// ```
    ///
    #[cfg(feature = "bincode")]
    pub fn new_bin<P: AsRef<Path>>(db_path: P, dump_policy: PickleDbDumpPolicy) -> PickleDb {
        PickleDb::new(db_path, dump_policy, SerializationMethod::Bin)
    }

    /// Constructs a new `PickleDb` instance that uses [YAML serialization](https://crates.io/crates/serde_yaml) for storing the data.
    ///
    /// # Arguments
    ///
    /// * `db_path` - a path where the DB will be stored
    /// * `dump_policy` - an enum value that determines the policy of dumping DB changes into the file. Please see
    ///    [PickleDb::load()](#method.load) to understand the different policy options
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use pickledb::{PickleDb, PickleDbDumpPolicy};
    ///
    /// let mut db = PickleDb::new_yaml("example.db", PickleDbDumpPolicy::AutoDump);
    /// ```
    ///
    #[cfg(feature = "yaml")]
    pub fn new_yaml<P: AsRef<Path>>(db_path: P, dump_policy: PickleDbDumpPolicy) -> PickleDb {
        PickleDb::new(db_path, dump_policy, SerializationMethod::Yaml)
    }

    /// Constructs a new `PickleDb` instance that uses [CBOR serialization](https://crates.io/crates/serde_cbor) for storing the data.
    ///
    /// # Arguments
    ///
    /// * `db_path` - a path where the DB will be stored
    /// * `dump_policy` - an enum value that determines the policy of dumping DB changes into the file. Please see
    ///    [PickleDb::load()](#method.load) to understand the different policy options
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use pickledb::{PickleDb, PickleDbDumpPolicy};
    ///
    /// let mut db = PickleDb::new_cbor("example.db", PickleDbDumpPolicy::AutoDump);
    /// ```
    ///
    #[cfg(feature = "cbor")]
    pub fn new_cbor<P: AsRef<Path>>(db_path: P, dump_policy: PickleDbDumpPolicy) -> PickleDb {
        PickleDb::new(db_path, dump_policy, SerializationMethod::Cbor)
    }

    /// Load a DB from a file.
    ///
    /// This method tries to load a DB from a file. Upon success an instance of `PickleDb` is returned,
    /// otherwise an [Error](error/struct.Error.html) object is returned.
    ///
    /// # Arguments
    ///
    /// * `db_path` - a path where the DB is loaded from
    /// * `dump_policy` - an enum value that determines the policy of dumping DB changes into the file.
    ///   The user can choose between the following options:
    ///   * [PickleDbDumpPolicy::NeverDump](enum.PickleDbDumpPolicy.html#variant.NeverDump) - never dump any change,
    ///     file will always remain read-only. When choosing this policy even calling to [dump()](#method.dump) won't dump the data.
    ///     Choosing this option is the same like calling [PickleDb::load_read_only()](#method.load_read_only)
    ///   * [PickleDbDumpPolicy::AutoDump](enum.PickleDbDumpPolicy.html#variant.AutoDump) - every change will be dumped
    ///     immediately and automatically to the file
    ///   * [PickleDbDumpPolicy::DumpUponRequest](enum.PickleDbDumpPolicy.html#variant.DumpUponRequest) - data won't be dumped
    ///     unless the user calls [dump()](#method.dump) proactively to dump the data
    ///   * [PickleDbDumpPolicy::PeriodicDump(Duration)](enum.PickleDbDumpPolicy.html#variant.PeriodicDump) - changes will be
    ///     dumped to the file periodically, no sooner than the Duration provided by the user. The way this mechanism works is
    ///     as follows: each time there is a DB change the last DB dump time is checked. If the time that has passed
    ///     since the last dump is higher than Duration, changes will be dumped, otherwise changes will not be dumped.
    /// * `serialization_method` - the serialization method used to store the data in the file
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
    ///
    /// let db = PickleDb::load("example.db", PickleDbDumpPolicy::AutoDump, SerializationMethod::Yaml);
    /// ```
    ///
    pub fn load<P: AsRef<Path>>(
        db_path: P,
        dump_policy: PickleDbDumpPolicy,
        serialization_method: SerializationMethod,
    ) -> Result<PickleDb> {
        let content = match fs::read(db_path.as_ref()) {
            Ok(file_content) => file_content,
            Err(err) => return Err(Error::new(ErrorCode::Io(err))),
        };

        let serializer = Serializer::new(serialization_method);

        let maps_from_file: (_, _) = match serializer.deserialize_db(&content) {
            Ok(maps) => maps,
            Err(err_str) => return Err(Error::new(ErrorCode::Serialization(err_str))),
        };

        let mut db_path_buf = PathBuf::new();
        db_path_buf.push(db_path);

        Ok(PickleDb {
            map: maps_from_file.0,
            list_map: maps_from_file.1,
            serializer,
            db_file_path: db_path_buf,
            dump_policy,
            last_dump: Instant::now(),
        })
    }

    /// Load a DB from a file stored in a Json format
    ///
    /// This method tries to load a DB from a file serialized in Json format. Upon success an instance of `PickleDb` is returned,
    /// otherwise an [Error](error/struct.Error.html) object is returned.
    ///
    /// # Arguments
    ///
    /// * `db_path` - a path where the DB is loaded from
    /// * `dump_policy` - an enum value that determines the policy of dumping DB changes into the file.
    ///   See [PickleDb::load()](#method.load) for more information
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use pickledb::{PickleDb, PickleDbDumpPolicy};
    ///
    /// let db = PickleDb::load_json("example.db", PickleDbDumpPolicy::AutoDump);
    /// ```
    ///
    #[cfg(feature = "json")]
    pub fn load_json<P: AsRef<Path>>(
        db_path: P,
        dump_policy: PickleDbDumpPolicy,
    ) -> Result<PickleDb> {
        PickleDb::load(db_path, dump_policy, SerializationMethod::Json)
    }

    /// Load a DB from a file stored in Bincode format
    ///
    /// This method tries to load a DB from a file serialized in Bincode format. Upon success an instance of `PickleDb` is returned,
    /// otherwise an [Error](error/struct.Error.html) object is returned.
    ///
    /// # Arguments
    ///
    /// * `db_path` - a path where the DB is loaded from
    /// * `dump_policy` - an enum value that determines the policy of dumping DB changes into the file.
    ///   See [PickleDb::load()](#method.load) for more information
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use pickledb::{PickleDb, PickleDbDumpPolicy};
    ///
    /// let db = PickleDb::load_bin("example.db", PickleDbDumpPolicy::AutoDump);
    /// ```
    ///
    #[cfg(feature = "bincode")]
    pub fn load_bin<P: AsRef<Path>>(
        db_path: P,
        dump_policy: PickleDbDumpPolicy,
    ) -> Result<PickleDb> {
        PickleDb::load(db_path, dump_policy, SerializationMethod::Bin)
    }

    /// Load a DB from a file stored in Yaml format
    ///
    /// This method tries to load a DB from a file serialized in Yaml format. Upon success an instance of `PickleDb` is returned,
    /// otherwise an [Error](error/struct.Error.html) object is returned.
    ///
    /// # Arguments
    ///
    /// * `db_path` - a path where the DB is loaded from
    /// * `dump_policy` - an enum value that determines the policy of dumping DB changes into the file.
    ///   See [PickleDb::load()](#method.load) for more information
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use pickledb::{PickleDb, PickleDbDumpPolicy};
    ///
    /// let db = PickleDb::load_yaml("example.db", PickleDbDumpPolicy::AutoDump);
    /// ```
    ///
    #[cfg(feature = "yaml")]
    pub fn load_yaml<P: AsRef<Path>>(
        db_path: P,
        dump_policy: PickleDbDumpPolicy,
    ) -> Result<PickleDb> {
        PickleDb::load(db_path, dump_policy, SerializationMethod::Yaml)
    }

    /// Load a DB from a file stored in Cbor format
    ///
    /// This method tries to load a DB from a file serialized in Cbor format. Upon success an instance of `PickleDb` is returned,
    /// otherwise an [Error](error/struct.Error.html) object is returned.
    ///
    /// # Arguments
    ///
    /// * `db_path` - a path where the DB is loaded from
    /// * `dump_policy` - an enum value that determines the policy of dumping DB changes into the file.
    ///   See [PickleDb::load()](#method.load) for more information
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use pickledb::{PickleDb, PickleDbDumpPolicy};
    ///
    /// let db = PickleDb::load_cbor("example.db", PickleDbDumpPolicy::AutoDump);
    /// ```
    ///
    #[cfg(feature = "cbor")]
    pub fn load_cbor<P: AsRef<Path>>(
        db_path: P,
        dump_policy: PickleDbDumpPolicy,
    ) -> Result<PickleDb> {
        PickleDb::load(db_path, dump_policy, SerializationMethod::Cbor)
    }

    /// Load a DB from a file in read-only mode.
    ///
    /// This method is similar to the [PickleDb::load()](#method.load) method with the only difference
    /// that the file is loaded from DB with a dump policy of
    /// [PickleDbDumpPolicy::NeverDump](enum.PickleDbDumpPolicy.html#variant.NeverDump), meaning
    /// changes will not be saved to the file, even when calling [dump()](#method.dump).
    /// Upon success an instance of `PickleDb` is returned, otherwise an [Error](error/struct.Error.html)
    /// object is returned.
    ///
    /// # Arguments
    ///
    /// * `db_path` - a path where the DB is loaded from
    /// * `serialization_method` - the serialization method used to store the data in the file
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use pickledb::{PickleDb, SerializationMethod};
    ///
    /// let mut readonly_db = PickleDb::load_read_only("example.db", SerializationMethod::Cbor).unwrap();
    ///
    /// // nothing happens by calling this method
    /// readonly_db.dump();
    /// ```
    pub fn load_read_only<P: AsRef<Path>>(
        db_path: P,
        serialization_method: SerializationMethod,
    ) -> Result<PickleDb> {
        PickleDb::load(db_path, PickleDbDumpPolicy::NeverDump, serialization_method)
    }

    /// Dump the data to the file.
    ///
    /// Calling this method is necessary only if the DB is loaded or created with a dump policy other than
    /// [PickleDbDumpPolicy::AutoDump](enum.PickleDbDumpPolicy.html#variant.AutoDump), otherwise the data
    /// is dumped to the file upon every change.
    ///
    /// This method returns `Ok` if dump is successful, Or an `Err(`[Error](error/struct.Error.html)`)` otherwise.
    ///
    pub fn dump(&mut self) -> Result<()> {
        if let PickleDbDumpPolicy::NeverDump = self.dump_policy {
            return Ok(());
        }

        match self.serializer.serialize_db(&self.map, &self.list_map) {
            Ok(ser_db) => {
                let temp_file_path = format!(
                    "{}.temp.{}",
                    self.db_file_path.to_str().unwrap(),
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs()
                );

                match fs::write(&temp_file_path, ser_db) {
                    Ok(_) => (),
                    Err(err) => return Err(Error::new(ErrorCode::Io(err))),
                }

                match fs::rename(temp_file_path, &self.db_file_path) {
                    Ok(_) => (),
                    Err(err) => return Err(Error::new(ErrorCode::Io(err))),
                }

                if let PickleDbDumpPolicy::PeriodicDump(_dur) = self.dump_policy {
                    self.last_dump = Instant::now();
                }
                Ok(())
            }
            Err(err_str) => Err(Error::new(ErrorCode::Serialization(err_str))),
        }
    }

    fn dumpdb(&mut self) -> Result<()> {
        match self.dump_policy {
            PickleDbDumpPolicy::AutoDump => self.dump(),
            PickleDbDumpPolicy::PeriodicDump(duration) => {
                let now = Instant::now();
                if now.duration_since(self.last_dump) > duration {
                    self.last_dump = Instant::now();
                    self.dump()?;
                }
                Ok(())
            }

            _ => Ok(()),
        }
    }

    /// Set a key-value pair.
    ///
    /// The key has to be a string but the value can be of any type that is serializable.
    /// That includes all primitive types, vectors, tuples, enums and every struct that
    /// has the `#[derive(Serialize, Deserialize)` attribute.
    ///
    /// This method returns `Ok` if set is successful, Or an `Err(`[Error](error/struct.Error.html)`)`
    /// otherwise. An error is not likely to happen but may occur mostly in cases where this
    /// action triggers a DB dump (which is decided according to the dump policy)
    ///
    /// # Arguments
    ///
    /// * `key` - a string key
    /// * `value` - a value of any serializable type
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use serde::{Serialize, Deserialize};
    /// # let mut db = pickledb::PickleDb::new_bin("1.db", pickledb::PickleDbDumpPolicy::AutoDump);
    /// // set a number
    /// db.set("key1", &100).unwrap();
    ///
    /// // set a floating point number
    /// db.set("key2", &1.234).unwrap();
    ///
    /// // set a String
    /// db.set("key3", &String::from("hello world")).unwrap();
    ///
    /// // set a Vec
    /// db.set("key4", &vec![1,2,3]).unwrap();
    ///
    /// // set a struct
    /// #[derive(Serialize, Deserialize)]
    /// struct Coor {
    ///     x: i32,
    ///     y: i32,
    /// }
    /// let mycoor = Coor { x: 1, y : 2 };
    /// db.set("key5", &mycoor).unwrap();
    /// ```
    ///
    pub fn set<V>(&mut self, key: &str, value: &V) -> Result<()>
    where
        V: Serialize,
    {
        if self.list_map.contains_key(key) {
            self.list_map.remove(key);
        }
        let ser_data = match self.serializer.serialize_data(value) {
            Ok(data) => data,
            Err(err_str) => return Err(Error::new(ErrorCode::Serialization(err_str))),
        };

        let original_value = self.map.insert(String::from(key), ser_data);
        match self.dumpdb() {
            Ok(_) => Ok(()),
            Err(err) => {
                match original_value {
                    None => {
                        self.map.remove(key);
                    }
                    Some(orig_value) => {
                        self.map.insert(String::from(key), orig_value.to_vec());
                    }
                }

                Err(err)
            }
        }
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
    /// ```no_run
    /// # use serde::{Serialize, Deserialize};
    /// # #[derive(Serialize, Deserialize)] struct Coor { x: i32, y: i32, }
    /// # let mut db = pickledb::PickleDb::new_bin("1.db", pickledb::PickleDbDumpPolicy::AutoDump);
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
    /// let vec = db.get::<Vec<i32>>("key4").unwrap();
    ///
    /// // read a struct
    /// let coor = db.get::<Coor>("key5").unwrap();
    /// ```
    ///
    pub fn get<V>(&self, key: &str) -> Option<V>
    where
        V: DeserializeOwned,
    {
        match self.map.get(key) {
            Some(val) => self.serializer.deserialize_data::<V>(val),
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
        [
            self.map
                .iter()
                .map(|(key, _)| key.clone())
                .collect::<Vec<String>>(),
            self.list_map
                .iter()
                .map(|(key, _)| key.clone())
                .collect::<Vec<String>>(),
        ]
        .concat()
    }

    /// Get the total number of keys in the DB.
    ///
    pub fn total_keys(&self) -> usize {
        self.map.iter().len() + self.list_map.iter().len()
    }

    /// Remove a key-value pair or a list from the DB.
    ///
    /// This methods returns `Ok(true)` if the key was found in the DB or `Ok(false)` if it wasn't found.
    /// It may also return `Err(`[Error](error/struct.Error.html)`)` if key was found but removal failed.
    /// Removal error is not likely to happen but may occur mostly in cases where this action triggers a DB dump
    /// (which is decided according to the dump policy)
    ///
    /// # Arguments
    ///
    /// * `key` - the key or list name to remove
    ///
    pub fn rem(&mut self, key: &str) -> Result<bool> {
        let remove_map = match self.map.remove(key) {
            None => None,
            Some(val) => match self.dumpdb() {
                Ok(_) => Some(val),
                Err(err) => {
                    self.map.insert(String::from(key), val);
                    return Err(err);
                }
            },
        };

        let remove_list = match self.list_map.remove(key) {
            None => None,
            Some(list) => match self.dumpdb() {
                Ok(_) => Some(list),
                Err(err) => {
                    self.list_map.insert(String::from(key), list);
                    return Err(err);
                }
            },
        };

        Ok(remove_map.is_some() || remove_list.is_some())
    }

    /// Create a new list.
    ///
    /// This method just creates a new list, it doesn't add any elements to it.
    /// If another list or value is already set under this key, they will be overridden,
    /// meaning the new list will override the old list or value.
    ///
    /// Upon success, the method returns an object of type
    /// [PickleDbListExtender](struct.PickleDbListExtender.html) that enables to add
    /// items to the newly created list. Alternatively you can use [ladd()](#method.ladd)
    /// or [lextend()](#method.lextend) to add items to the list.
    ///
    /// In case of a failure an
    /// `Err(`[Error](error/struct.Error.html)`)` is returned. Failures
    /// are not likely to happen but may occur mostly in cases where this action triggers a DB dump
    /// (which is decided according to the dump policy)
    ///
    /// # Arguments
    ///
    /// * `name` - the key of the list that will be created
    ///
    pub fn lcreate(&mut self, name: &str) -> Result<PickleDbListExtender> {
        let new_list: Vec<Vec<u8>> = Vec::new();
        if self.map.contains_key(name) {
            self.map.remove(name);
        }
        self.list_map.insert(String::from(name), new_list);
        self.dumpdb()?;
        Ok(PickleDbListExtender {
            db: self,
            list_name: String::from(name),
        })
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
    ///
    /// If the item was added successfully the method returns
    /// `Some(`[PickleDbListExtender](struct.PickleDbListExtender.html)`)` which enables to add more
    /// items to the list. Alternatively the method returns `None` if the list isn't found in the DB
    /// or if a failure happened while extending the list. Failures are not likely to happen but may
    /// occur mostly in cases where this action triggers a DB dump (which is decided according to the dump policy)
    ///
    /// # Arguments
    ///
    /// * `name` - the list key
    /// * `value` - a reference of the item to add to the list
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # let mut db = pickledb::PickleDb::new_bin("1.db", pickledb::PickleDbDumpPolicy::AutoDump);
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
        V: Serialize,
    {
        self.lextend(name, &[value])
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
    ///
    /// If all items were added successfully the method returns
    /// `Some(`[PickleDbListExtender](struct.PickleDbListExtender.html)`)` which enables to add more
    /// items to the list. Alternatively the method returns `None` if the list isn't found in the DB
    /// or if a failure happened while extending the list. Failures are not likely to happen but may
    /// occur mostly in cases where this action triggers a DB dump (which is decided according to the dump policy)
    ///
    /// # Arguments
    ///
    /// * `name` - the list key
    /// * `seq` - an iterator containing references to the new items to add to the list
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # let mut db = pickledb::PickleDb::new_bin("1.db", pickledb::PickleDbDumpPolicy::AutoDump);
    /// // create a new list
    /// db.lcreate("list1");
    ///
    /// // add a bunch of numbers to the list
    /// db.lextend("list1", &vec![100, 200, 300]).unwrap()
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
    pub fn lextend<'a, V, I>(&mut self, name: &str, seq: I) -> Option<PickleDbListExtender>
    where
        V: 'a + Serialize,
        I: IntoIterator<Item = &'a V>,
    {
        let serializer = &self.serializer;
        match self.list_map.get_mut(name) {
            Some(list) => {
                let original_len = list.len();
                let serialized: Vec<Vec<u8>> = seq
                    .into_iter()
                    .map(|x| serializer.serialize_data(x).unwrap())
                    .collect();
                list.extend(serialized);
                match self.dumpdb() {
                    Ok(_) => (),
                    Err(_) => {
                        let same_list = self.list_map.get_mut(name).unwrap();
                        same_list.truncate(original_len);
                        return None;
                    }
                }
                Some(PickleDbListExtender {
                    db: self,
                    list_name: String::from(name),
                })
            }

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
    /// ```no_run
    /// # let mut db = pickledb::PickleDb::new_bin("1.db", pickledb::PickleDbDumpPolicy::AutoDump);
    /// // create a list
    /// db.lcreate("list1");
    ///
    /// // add a number to list1
    /// db.ladd("list1", &100);
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
        V: DeserializeOwned,
    {
        match self.list_map.get(name) {
            Some(list) => match list.get(pos) {
                Some(val) => self.serializer.deserialize_data::<V>(val),
                None => None,
            },
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
            None => 0,
        }
    }

    /// Remove a list.
    ///
    /// This method is somewhat similar to [rem()](#method.rem) but with 2 small differences:
    /// * This method only removes lists and not key-value pairs
    /// * The return value of this method is the number of items that were in
    ///   the list that was removed. If the list doesn't exist a value of zero (0) is
    ///   returned. In case of a failure an `Err(`[Error](error/struct.Error.html)`)` is returned.
    ///   Failures are not likely to happen but may occur mostly in cases where this action triggers a
    ///   DB dump (which is decided according to the dump policy)
    ///
    /// # Arguments
    ///
    /// * `name` - the list key to remove
    ///
    pub fn lrem_list(&mut self, name: &str) -> Result<usize> {
        let res = self.llen(name);
        match self.list_map.remove(name) {
            Some(list) => match self.dumpdb() {
                Ok(_) => Ok(res),
                Err(err) => {
                    self.list_map.insert(String::from(name), list);
                    Err(err)
                }
            },
            None => Ok(res),
        }
    }

    /// Pop an item out of a list.
    ///
    /// This method takes a list name and a position inside the list, removes the
    /// item in this position and returns it to the user. It's the user's responsibility
    /// to know what is the correct type of the item and give it while calling this method.
    /// Since the item in the lists are stored in a serialized way the returned object
    /// is not a reference to the item stored in a DB but actually a new instance of it.
    ///
    /// If the list is not found in the DB or the given position is out of bounds no item
    /// will be removed and `None` will be returned. `None` may also be returned
    /// if removing the item fails, which may happen mostly in cases where this action
    /// triggers a DB dump (which is decided according to the dump policy).
    /// Otherwise the item will be removed and `Some(V)` will be returned.
    ///
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
    /// ```no_run
    /// # let mut db = pickledb::PickleDb::new_bin("1.db", pickledb::PickleDbDumpPolicy::AutoDump);
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
        V: DeserializeOwned,
    {
        match self.list_map.get_mut(name) {
            Some(list) => {
                if pos < list.len() {
                    let res = list.remove(pos);
                    match self.dumpdb() {
                        Ok(_) => self.serializer.deserialize_data::<V>(&res),
                        Err(_) => {
                            let same_list = self.list_map.get_mut(name).unwrap();
                            same_list.insert(pos, res);
                            None
                        }
                    }
                } else {
                    None
                }
            }

            None => None,
        }
    }

    /// Remove an item out of a list.
    ///
    /// This method takes a list name and a reference to a value, removes the first instance of the
    /// value if it exists in the list, and returns an indication whether the item was removed or not.
    ///
    /// If the list is not found in the DB or the given value isn't found in the list, no item will
    /// be removed and `Ok(false)` will be returned.
    /// If removing the item fails, which may happen mostly in cases where this action triggers
    /// a DB dump (which is decided according to the dump policy), an
    /// `Err(`[Error](error/struct.Error.html)`)` is returned.
    /// Otherwise the item will be removed and `Ok(true)` will be returned.
    ///
    /// This method is very similar to [lpop()](#method.lpop), the only difference is that this
    /// methods returns an indication and [lpop()](#method.lpop) returns the actual item that was removed.
    ///
    /// # Arguments
    ///
    /// * `name` - the list key
    /// * `value` - the item to remove
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # let mut db = pickledb::PickleDb::new_bin("1.db", pickledb::PickleDbDumpPolicy::AutoDump);
    /// // create a list
    /// db.lcreate("list1");
    ///
    /// // add 4 items to the list
    /// db.lextend("list1", &vec![1,2,3,4]);
    ///
    /// // remove 2
    /// db.lrem_value("list1", &2).unwrap();
    ///
    /// // The list now looks like this: [1, 3, 4]
    ///
    /// // remove 3
    /// db.lrem_value("list1", &3).unwrap();
    ///
    /// // The list now looks like this: [1, 4]
    /// ```
    ///
    pub fn lrem_value<V>(&mut self, name: &str, value: &V) -> Result<bool>
    where
        V: Serialize,
    {
        match self.list_map.get_mut(name) {
            Some(list) => {
                let serialized_value = match self.serializer.serialize_data(&value) {
                    Ok(val) => val,
                    Err(err_str) => return Err(Error::new(ErrorCode::Serialization(err_str))),
                };

                match list.iter().position(|x| *x == serialized_value) {
                    Some(pos) => {
                        list.remove(pos);
                        match self.dumpdb() {
                            Ok(_) => Ok(true),
                            Err(err) => {
                                let same_list = self.list_map.get_mut(name).unwrap();
                                same_list.insert(pos, serialized_value);
                                Err(err)
                            }
                        }
                    }

                    None => Ok(false),
                }
            }

            None => Ok(false),
        }
    }

    /// Return an iterator over the keys and values in the DB.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # let mut db = pickledb::PickleDb::new_bin("1.db", pickledb::PickleDbDumpPolicy::AutoDump);
    /// # type Rectangle = usize;
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
        PickleDbIterator {
            map_iter: self.map.iter(),
            serializer: &self.serializer,
        }
    }

    /// Return an iterator over the items in certain list.
    ///
    /// # Arguments
    ///
    /// * `name` - the list name. If the list doesn't exist an exception is thrown
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # let mut db = pickledb::PickleDb::new_bin("1.db", pickledb::PickleDbDumpPolicy::AutoDump);
    /// // create a new list
    /// db.lcreate("list1").unwrap()
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
            Some(list) => PickleDbListIterator {
                list_iter: list.iter(),
                serializer: &self.serializer,
            },
            None => panic!("List '{}' doesn't exist", name),
        }
    }
}

impl Drop for PickleDb {
    fn drop(&mut self) {
        if !matches!(
            self.dump_policy,
            PickleDbDumpPolicy::NeverDump | PickleDbDumpPolicy::DumpUponRequest
        ) {
            // try to dump, ignore if fails
            let _ = self.dump();
        }
    }
}
