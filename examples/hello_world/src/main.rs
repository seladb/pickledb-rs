//! A simple example of how to use PickleDB. It includes:
//! * Creating a new DB
//! * Loading am existing DB from a file
//! * Setting and getting key-value pairs of different types
//! 
#[macro_use]
extern crate serde_derive;

use pickledb::{PickleDb, PickleDbDumpPolicy};
use std::fmt::{self, Formatter, Display};

/// Define an example struct which represnets a rectangle. 
/// Next we'll show how to write and read it into the DB.
#[derive(Serialize, Deserialize)]
struct Rectangle {
    width: i32,
    length: i32
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Rectangle: length={}, width={}", self.length, self.width)
    }
}

fn main() {

    // create a new DB with AutoDum, meaning every change is written to the file
    let mut db = PickleDb::new("example.db", PickleDbDumpPolicy::AutoDump);
    
    // set the value 100 to the key 'key1'
    db.set("key1", &100);

    // set the value 1.1 to the key 'key2'
    db.set("key2", &1.1);

    // set the value 'hello world' to the key 'key3'
    db.set("key3", &String::from("hello world"));

    // set a vector value to the key 'key4'
    db.set("key4", &vec![1,2,3]);

    // set a Rectangle value to the key 'key5'
    db.set("key5", &Rectangle { width: 4, length: 10});

    // print the value of key1
    println!("The value of key1 is: {}", db.get::<i32>("key1").unwrap());

    // print the value of key2
    println!("The value of key2 is: {}", db.get::<f32>("key2").unwrap());

    // print the value of key3
    println!("The value of key3 is: {}", db.get::<String>("key3").unwrap());

    // print the value of key4
    println!("The value of key4 is: {:?}", db.get::<Vec<i32>>("key4").unwrap());

    // print the value of key5
    println!("The value of key5 is: {}", db.get::<Rectangle>("key5").unwrap());


    // override the value of key1. Please note the new value is of a different type the former one
    db.set("key1", &String::from("override"));

    // print the value of key1
    println!("The value of key1 is: {}", db.get::<String>("key1").unwrap());

    
    // remove key2
    db.rem("key2");

    // was key2 removed?
    println!("key2 was removed. Is it still in the db? {}", db.get::<f32>("key2").is_some());


    // load an existing DB from a file (the same file in this case)
    let db2 = PickleDb::load("example.db", PickleDbDumpPolicy::DumpUponRequest).unwrap();

    // print the value of key1
    println!("Value of key1 as loaded from file is: {}", db2.get::<String>("key1").unwrap());
}
