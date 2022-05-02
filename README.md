# PickleDB

[![Rust test](https://github.com/seladb/pickledb-rs/workflows/Rust%20test/badge.svg)](https://github.com/seladb/pickledb-rs/actions?query=workflow%3A%22Rust+test%22)
[![Rust audit](https://github.com/seladb/pickledb-rs/workflows/Rust%20audit/badge.svg)](https://github.com/seladb/pickledb-rs/actions?query=workflow%3A%22Rust+audit%22)
[![Crate](https://img.shields.io/crates/v/pickledb.svg)](https://crates.io/crates/pickledb)
[![API](https://docs.rs/pickledb/badge.svg)](https://docs.rs/pickledb)

PickleDB is a lightweight and simple key-value store written in Rust, heavily inspired by [Python's PickleDB](https://pythonhosted.org/pickleDB/)

## PickleDB is fun and easy to use

```rust
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};

fn main() {

    // create a new DB with AutoDump (meaning every change is written to the file)
    // and with Json serialization (meaning DB will be dumped to file as a Json object)
    let mut db = PickleDb::new("example.db", PickleDbDumpPolicy::AutoDump, SerializationMethod::Json);

    // set the value 100 to the key 'key1'
    db.set("key1", &100).unwrap();

    // print the value of key1
    println!("The value of key1 is: {}", db.get::<i32>("key1").unwrap());

    // load the DB from the same file
    let db2 = PickleDb::load("example.db", PickleDbDumpPolicy::DumpUponRequest, SerializationMethod::Json).unwrap();

    // print the value of key1
    println!("The value of key1 as loaded from file is: {}", db2.get::<i32>("key1").unwrap());
}
```

## Installation

This crate works with Cargo and can be found in [crates.io](https://crates.io/crates/pickledb)
Add this to your `Cargo.toml`:

```toml
[dependencies]
pickledb = "0.5.1"
```

## Documentation

All documentation for this crate can be found in [docs.rs](https://docs.rs/pickledb)

## Examples

There are currently two examples shipped with PickleDB:

- [Hello World](https://github.com/seladb/pickledb-rs/tree/master/examples/hello_world) which shows the basic usage of PickleDB: 
  create a new DB, load a DB from file, get/set key-value pairs of different types, and more
- [Lists](https://github.com/seladb/pickledb-rs/tree/master/examples/lists) which shows how to use lists in PickleDB: 
  create new lists, add/remove items from lists, retrieve items from lists, remove lists, and more

## Changelog

### Version 0.5.1

- Bugfix: Add missing JSON feature gate

### Version 0.5.0

- Turn on/off file formats with features
- `DumpUponRequest` policy no longer dumps on `Drop`
- (internal) Switch CI from TravisCI to GitHub Actions

### Version 0.4.1

- Bump up dependencies versions to fix vulnerabilities found in few of them

### Version 0.4.0

- Changed all doc tests from `ignore` to `no_run` so generated docs don't contain untested warnings
- Changed both instances of `lextend` to take iterators of references rather than a slice of values
- Fixed bug in `load_test()`
- Fixed rustfmt and clippy warnings
- Added examples to `Cargo.toml` to allow them to be run via Cargo

### Version 0.3.0

- Added new serialization options. Now PickleDB supports [JSON](https://crates.io/crates/serde_json), [Bincode](https://crates.io/crates/bincode),
  [YAML](https://crates.io/crates/serde_yaml) and [CBOR](https://crates.io/crates/serde_cbor) serializations
- Added proper error handling ([Issue #3](https://github.com/seladb/pickledb-rs/issues/3))
- Use `Path` and `PathBuf` instead of strings to describe DB paths
- Better organization of the code

### Version 0.2.0

- Dump the DB to file in a crash-safe manner using a temp file (Thanks jamwt from Reddit
  for the tip: https://www.reddit.com/r/rust/comments/agumun/check_out_pickledb_a_lightweight_and_simple/ee987j0)
- Extend lists became easier and multiple calls to `lcreate()`, `ladd()` and `lextend()` can be chained
- Added an iterator over keys and values in the DB
- Added an iterator over items in a list
