PickleDB Rust
=============

[![Build Status](https://api.travis-ci.org/seladb/pickledb-rs.svg?branch=master)](https://travis-ci.org/seladb/pickledb-rs)

PickleDB-rs is a lightweight and simple key-value store. It is a Rust version for [Python's PickleDB](https://pythonhosted.org/pickleDB/)

## PickleDB is fun and easy to use

```rust
use pickledb::PickleDb;

fn main() {
    
    // create a new DB with auto_dump=true, meaning each change is written 
    // to the file
    let mut db = PickleDb::new("example.db", true);
    
    // set the value 100 to the key 'key1'
    db.set("key1", &100);
    
    // print the value of key1
    println!("Value of key1 is: {}", db.get::<String>("key1"));

    // load the same DB from a file
    let read_only_db = PickleDb::load("example.db", false);
    
    // print the value of key1
    println!("Value of key1 as loaded from file is: {}", read_only_db.get::<String>("key1"));
}
```
