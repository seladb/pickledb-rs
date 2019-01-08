use pickledb::PickleDb;
use std::iter;
use std::collections::HashMap;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use rand::seq::SliceRandom;

mod common;

extern crate serde_derive;

#[test]
fn lists_and_values() {

}


fn gen_random_string<T: Rng>(rng: &mut T, size: usize) -> String {
    iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .take(size)
        .collect()
}

#[test]
fn load_test() {
    set_test_rsc!("load_test.db");

    let mut db = PickleDb::new("load_test.db", false);

    // number of keys to generate
    let generate_keys = 1000;

    // a map that will store all generated keys and the value type
    let mut map: HashMap<String, &str> = HashMap::new();

    // generate keys and values
    for _ in 0..generate_keys {
        let mut rng = thread_rng();

        // generate key: an alphanumeric key of size 3..15
        let mut key_len: u32 = rng.gen_range(3, 15);
        let mut key: String = gen_random_string(&mut rng, key_len as usize);

        // if key already exists, generate another one
        while map.get(&key).is_some() {
            key_len = rng.gen_range(3, 15);
            key = gen_random_string(&mut rng, key_len as usize);
        }

        // possible value types: [i32, f32, String, Vec, List]
        let possible_value_types = [1,2,3,4,5];

        // randomly choose a type
        match possible_value_types.choose(&mut rng).unwrap() {
            1 => { // add a i32 value
                db.set(&key, &rng.gen::<i32>());
                map.insert(String::from(key), "i32");
            },
            2 => { // add a f32 value
                db.set(&key, &rng.gen::<f32>());
                map.insert(String::from(key), "f32");
            },
            3 => { // add a String value
                let val_size = rng.gen_range(1, 50);
                db.set(&key, &gen_random_string(&mut rng, val_size));
                map.insert(String::from(key), "string");
            },
            4 => { // add a Vec<i32> value
                // randomize vec size 1..10
                let vec_size = rng.gen_range(1, 10);
                let mut vec = vec![];

                // randomize vec values
                for _ in 1..vec_size {
                    vec.push(rng.gen::<i32>());
                }
                db.set(&key, &vec);
                map.insert(String::from(key), "vec");
            },
            5 => { // add a List value

                // all list keys get a "list_" prefix
                let mut list_key = key.clone();
                list_key.insert_str(0, "list_");

                // create the list
                db.lcreate(&list_key);

                map.insert(String::from(list_key.clone()), "list");

                // randomize list size 1..50
                let list_size: u32 = rng.gen_range(1, 50);

                // create list elements
                for _ in 1..list_size {

                    // randomly choose an element type: [i32, f32, String, Vec]
                    match possible_value_types[..4].choose(&mut rng).unwrap() {
                        1 => { // add a i32 element to the list
                            assert!(db.ladd(&list_key, &rng.gen::<i32>()))
                        },
                        2 => { // add a f32 element to the list
                            assert!(db.ladd(&list_key, &rng.gen::<f32>()));
                        }, 
                        3 => { // add a String element to the list
                            let val_size = rng.gen_range(1, 50);
                            assert!(db.ladd(&list_key, &gen_random_string(&mut rng, val_size)));
                        },
                        4 => { // add a Vec<i32> element to the list
                            // randomize vec size 1..10
                            let vec_size = rng.gen_range(1, 10);
                            let mut vec = vec![];

                            // randomize vec values
                            for _ in 1..vec_size {
                                vec.push(rng.gen::<i32>());
                            }

                            // add vec as an element to the list
                            assert!(db.ladd(&list_key, &vec));
                        },
                        _ => panic!("Cannot add list inside a list!"),
                    }
                }
            },
            _ => panic!("Chosen wrong type to generate!"),
        }
    }

    // verify that the number of values we wanted to generate equals to actually generated
    assert_eq!(map.iter().len(), generate_keys);

    // dump DB to file
    db.dump();
    
    // read again from file
    let read_db = PickleDb::load("load_test.db", false).unwrap();

    // iterate every key/value_type in map saved before
    for (key, val_type) in map.iter() {
        
        // verify key exists in db
        assert!(read_db.exists(&key), format!("Key {} of type {} isn't found", key, val_type));

        // get the value according to the value_type saved
        match val_type {
            &"i32" => assert!(read_db.get::<i32>(&key).is_some()),
            &"f32" => assert!(read_db.get::<f32>(&key).is_some()),
            &"string" => assert!(read_db.get::<String>(&key).is_some()),
            &"vec" => assert!(read_db.get::<Vec<i32>>(&key).is_some()),
            &"list" => assert!(read_db.lexists(&key)),
            _ => (),
        }
    }

    // check that the total number of keys in db equals to number of keys generated
    assert_eq!(read_db.total_keys(), generate_keys);
}