//! PickleDB
//! ========
//!
//! PickleDB-rs is a lightweight and simple key-value store written in Rust, heavily inspired by [Python's PickleDB](https://pythonhosted.org/pickleDB/)
//!
//! PickleDB's architecture is very simple and straight-forward: the whole key-value data structure is stored in memory and is dumped to a file
//! periodically according to a policy defined by the user. There are APIs to create a new key-value store in memory or to load it from a file.
//! Everything runs in the user's process and memory and in the same thread, which means that the key-value data will be stored in the user
//! process's memory and each API call will access that key-value store directly and may trigger a dump to the DB file. There are no additional
//! threads or processes created throughout the life-cycle of any of the APIs.
//!
//! ## So what is it useful for?
//!
//! Basically for any use case that needs a simple and relatively small key-value store that can run in-process and
//! be stored in a file. Most of the key-value stores out there provide high scalability, performance and robustness, but in the cost of a very
//! complex architecture, a lot of installation and configuration, and in many cases require a descent amount of resources.
//! But sometimes you don't need this scalability and performance and all you need is a simple solution that can be easily set up and is easy to
//! use and understand. That's where PickleDB-rs comes into the picture! I personally encountered several use cases like that and that's how I came
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
//! * Structs and Enums that are serializable (please read more below)
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
//! In order to achieve this magic, all objects must be serializable. PickleDB uses the [Serde](https://serde.rs/) library for serialization.
//! Currently 4 types of serialization are supported:
//! * [JSON serialization](https://crates.io/crates/serde_json)
//! * [Bincode serialization](https://crates.io/crates/bincode)
//! * [YAML serialization](https://crates.io/crates/serde_yaml)
//! * [CBOR serialization](https://crates.io/crates/serde_cbor)
//!
//! The serialization types are enabled and disabled with features (`json` (enabled by default), `bincode`, `yaml`, and `cbor`).
//! To enable them, just add their names to the `features` list when declaring the dependency. To disable JSON, set `default-features` to false.
//! For instance, `pickledb = { version = "0.5", features = ["cbor", "yaml"], default-features = false }` would enable CBOR and YAML only.
//!
//! The user can choose a serialization type to use upon creating a DB or loading it from a file.
//!
//! So what does it mean that all objects must be serializable? That means that all objects that you use must be serializable.
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
pub use self::extenders::PickleDbListExtender;
pub use self::iterators::{
    PickleDbIterator, PickleDbIteratorItem, PickleDbListIterator, PickleDbListIteratorItem,
};
pub use self::pickledb::{PickleDb, PickleDbDumpPolicy};
pub use self::serialization::SerializationMethod;

mod extenders;
mod iterators;
mod pickledb;
mod serialization;

pub mod error;
