PickleDB
========

[![Build Status](https://api.travis-ci.org/seladb/pickledb-rs.svg?branch=master)](https://travis-ci.org/seladb/pickledb-rs)
[![Crate](https://img.shields.io/crates/v/pickledb.svg)](https://crates.io/crates/pickledb)
[![API](https://docs.rs/pickledb/badge.svg)](https://docs.rs/pickledb)

PickleDB is a lightweight and simple key-value store written in Rust, heavily inspired by [Python's PickleDB](https://pythonhosted.org/pickleDB/)

## PickleDB is fun and easy to use

```rust
use pickledb::PickleDb;

fn main() {
    
    // create a new DB with AutoDum, meaning every change is written to the file
    let mut db = PickleDb::new("example.db", PickleDbDumpPolicy::AutoDump);
    
    // set the value 100 to the key 'key1'
    db.set("key1", &100);
    
    // print the value of key1
    println!("The value of key1 is: {}", db.get::<i32>("key1").unwrap());

    // load the DB from the same file
    let db2 = PickleDb::load("example.db", PickleDbDumpPolicy::DumpUponRequest).unwrap();

    // print the value of key1
    println!("The value of key1 as loaded from file is: {}", db2.get::<i32>("key1").unwrap());
}
```

## Installation

This crate works with Cargo and can be found in [crates.io](https://crates.io/crates/pickledb)
Add this to your `Cargo.toml`:

```toml
[dependencies]
pickledb = "0.2.0"
```

## Documentation

All documentation for this crate can be found in [docs.rs](https://docs.rs/pickledb)

## Examples

There are currently two examples shipped with PickleDB:

* [Hello World](https://github.com/seladb/pickledb-rs/tree/master/examples/hello_world) which shows the basic usage of PickleDB: 
  create a new DB, load a DB from file, get/set key-value pairs of different types, and more
* [Lists](https://github.com/seladb/pickledb-rs/tree/master/examples/lists) which shows how to use lists in PickleDB: 
  create new lists, add/remove items from lists, retrieve items from lists, remove lists, and more