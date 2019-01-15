PickleDB Rust
=============

[![Build Status](https://api.travis-ci.org/seladb/pickledb-rs.svg?branch=master)](https://travis-ci.org/seladb/pickledb-rs)

PickleDB-rs is a lightweight and simple key-value store written in Rust, heavily inspired by [Python's PickleDB](https://pythonhosted.org/pickleDB/)

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

This crate works with Cargo and can be found in [crates.io](https://crates.io/crates/pickledb-rs)
Add this to your `Cargo.toml`:

```toml
[dependencies]
pickledb-rs = "*"
```

## Documentation

All documentation for this crate can be found in [docs.rs](https://docs.rs/pickledb-rs)