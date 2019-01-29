//! An example of how to use lists in PickleDB. It includes:
//! * Creating a new DB
//! * Loading am existing DB from a file
//! * Creating and removing lists
//! * Adding and removing items of different type to lists
//! * Retrieving list items
//! 
#[macro_use]
extern crate serde_derive;

use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use std::fmt::{self, Formatter, Display};

/// Define an example struct which represents a rectangle. 
/// Next we'll show how to use it in lists.
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

/// Create a new DB and add one key-value pair to it
fn create_db(db_name: &str) {
    let mut new_db = PickleDb::new(db_name, PickleDbDumpPolicy::AutoDump, SerializationMethod::Bin);

    new_db.set("key1", &100);
}

fn main() {
    
    // create a new DB
    create_db("example.db");

    // load the DB
    let mut db = PickleDb::load("example.db", PickleDbDumpPolicy::AutoDump, SerializationMethod::Bin).unwrap();

    // print the existing value in key1
    println!("The value of key1 is: {}", db.get::<i32>("key1").unwrap());

    // create a new list
    db.lcreate("list1")

    // add an integer item to the list
      .ladd(&200)

    // add an floating point item to the list
      .ladd(&2.1)

    // add a string to the list
      .ladd(&String::from("my list"))

    // add a vector of chars to the list
      .ladd(&vec!['a', 'b', 'c'])

    // add multiple values to the list: add 3 rectangles 
      .lextend(&vec![
        Rectangle { width: 2, length: 4}, 
        Rectangle { width: 10, length: 22},
        Rectangle { width: 1, length: 22}, 
        ]);

    // print the list length
    println!("list1 length is: {}", db.llen("list1"));

    // print the item in each position of the list
    println!("list1[0] = {}", db.lget::<i32>("list1", 0).unwrap());
    println!("list1[1] = {}", db.lget::<f64>("list1", 1).unwrap());
    println!("list1[2] = {}", db.lget::<String>("list1", 2).unwrap());
    println!("list1[3] = {:?}", db.lget::<Vec<char>>("list1", 3).unwrap());
    println!("list1[4] = {}", db.lget::<Rectangle>("list1", 4).unwrap());
    println!("list1[6] = {}", db.lget::<Rectangle>("list1", 5).unwrap());
    println!("list1[7] = {}", db.lget::<Rectangle>("list1", 6).unwrap());

    // remove an item in the list
    db.lpop::<i32>("list1", 0);

    // print the new first item of the list
    println!("The new list1[0] = {}", db.lget::<f64>("list1", 0).unwrap());

    // remove the entire list
    db.lrem_list("list1");

    // was list1 removed?
    println!("list1 was removed. Is it still in the db? {}", db.lexists("list1"));


    // create a new list
    db.lcreate("list2")
      .lextend(&vec![1,2,3,4]);

    // iterate over the items in list2
    for item_iter in db.liter("list2") {
        println!("Current item is: {}", item_iter.get_item::<i32>().unwrap());
    }
}
