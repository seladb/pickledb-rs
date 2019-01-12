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
    println!("Value of key1 is: {}", db.get::<i32>("key1"));

    // load the DB from the same file
    let db2 = PickleDb::load("example.db", PickleDbDumpPolicy::DumpUponRequest);

    // print the value of key1
    println!("Value of key1 as loaded from file is: {}", db2.get::<i32>("key1"));
}
```
