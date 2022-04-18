use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use rand::distributions::Alphanumeric;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::iter;

mod common;

#[cfg(test)]
extern crate rstest;

use rstest::rstest_parametrize;

#[rstest_parametrize(ser_method_int, case(0), case(1), case(2), case(3))]
fn lists_and_values(ser_method_int: i32) {
    test_setup!("lists_and_values", ser_method_int, db_name);

    let mut db = PickleDb::new(
        &db_name,
        PickleDbDumpPolicy::AutoDump,
        ser_method!(ser_method_int),
    );

    // set a few values
    assert!(db.set("key1", &String::from("val1")).is_ok());
    assert!(db.set("key2", &1).is_ok());
    assert!(db.set("key3", &vec![1, 2, 3]).is_ok());

    // create a few lists and add values to them
    db.lcreate("list1").unwrap().lextend(&[1, 2, 3]);

    db.lcreate("list2")
        .unwrap()
        .ladd(&1.1)
        .ladd(&String::from("some val"));

    // read keys and lists
    #[allow(clippy::float_cmp)]
    {
        let read_db = PickleDb::load_read_only(&db_name, ser_method!(ser_method_int)).unwrap();
        assert_eq!(read_db.get::<String>("key1").unwrap(), String::from("val1"));
        assert_eq!(read_db.get::<i32>("key2").unwrap(), 1);
        assert_eq!(read_db.get::<Vec<i32>>("key3").unwrap(), vec![1, 2, 3]);
        assert_eq!(read_db.lget::<i32>("list1", 0).unwrap(), 1);
        assert_eq!(read_db.lget::<i32>("list1", 2).unwrap(), 3);
        assert_eq!(read_db.lget::<f64>("list2", 0).unwrap(), 1.1);
        assert_eq!(
            read_db.lget::<String>("list2", 1).unwrap(),
            String::from("some val")
        );
    }

    // create key and list with the same name, make sure they override one another
    assert!(db.set("key_or_list1", &1).is_ok());
    assert!(db.lcreate("key_or_list1").is_ok());

    assert!(db.exists("key_or_list1"));
    assert!(db.get::<i32>("key_or_list1").is_none());
    assert!(db.lexists("key_or_list1"));

    // now set the key again and verify list is removed
    assert!(db.set("key_or_list1", &2).is_ok());

    assert!(db.exists("key_or_list1"));
    assert_eq!(db.get::<i32>("key_or_list1").unwrap(), 2);
    assert!(!db.lexists("key_or_list1"));
}

fn gen_random_string<T: Rng>(rng: &mut T, size: usize) -> String {
    iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .take(size)
        .collect()
}

#[allow(clippy::cognitive_complexity)]
#[rstest_parametrize(ser_method_int, case(0), case(1), case(2), case(3))]
fn load_test(ser_method_int: i32) {
    test_setup!("load_test", ser_method_int, db_name);

    let mut db = PickleDb::new(
        &db_name,
        PickleDbDumpPolicy::DumpUponRequest,
        ser_method!(ser_method_int),
    );

    // number of keys to generate
    let generate_keys = 1000;

    // a map that will store all generated keys and the value type
    let mut map: HashMap<String, &str> = HashMap::new();

    // all list keys get a "list_" prefix
    let make_list_key = |key: &str| format!("list_{}", key);

    // generate keys and values
    for _ in 0..generate_keys {
        let mut rng = thread_rng();

        // generate key: an alphanumeric key of size 3..15
        let mut key_len: u32 = rng.gen_range(3, 15);
        let mut key: String = gen_random_string(&mut rng, key_len as usize);

        // if key already exists, generate another one
        while map.contains_key(&key) || map.contains_key(&make_list_key(&key)) {
            key_len = rng.gen_range(3, 15);
            key = gen_random_string(&mut rng, key_len as usize);
        }

        // possible value types: [i32, f32, String, Vec, List]
        let possible_value_types = [1, 2, 3, 4, 5];

        // randomly choose a type
        match possible_value_types.choose(&mut rng).unwrap() {
            1 => {
                // add a i32 value
                db.set(&key, &rng.gen::<i32>()).unwrap();
                assert!(map.insert(key, "i32").is_none());
            }
            2 => {
                // add a f32 value
                db.set(&key, &rng.gen::<f32>()).unwrap();
                assert!(map.insert(key, "f32").is_none());
            }
            3 => {
                // add a String value
                let val_size = rng.gen_range(1, 50);
                db.set(&key, &gen_random_string(&mut rng, val_size))
                    .unwrap();
                assert!(map.insert(key, "string").is_none());
            }
            4 => {
                // add a Vec<i32> value
                // randomize vec size 1..10
                let vec_size = rng.gen_range(1, 10);
                let mut vec = vec![];

                // randomize vec values
                for _ in 1..vec_size {
                    vec.push(rng.gen::<i32>());
                }
                db.set(&key, &vec).unwrap();
                assert!(map.insert(key, "vec").is_none());
            }
            5 => {
                // add a List value

                let list_key = make_list_key(&key);

                // create the list
                db.lcreate(&list_key).unwrap();

                assert!(map.insert(list_key.clone(), "list").is_none());

                // randomize list size 1..50
                let list_size: u32 = rng.gen_range(1, 50);

                // create list elements
                for _ in 1..list_size {
                    // randomly choose an element type: [i32, f32, String, Vec]
                    match possible_value_types[..4].choose(&mut rng).unwrap() {
                        1 => {
                            // add a i32 element to the list
                            assert!(db.ladd(&list_key, &rng.gen::<i32>()).is_some());
                        }
                        2 => {
                            // add a f32 element to the list
                            assert!(db.ladd(&list_key, &rng.gen::<f32>()).is_some());
                        }
                        3 => {
                            // add a String element to the list
                            let val_size = rng.gen_range(1, 50);
                            assert!(db
                                .ladd(&list_key, &gen_random_string(&mut rng, val_size))
                                .is_some());
                        }
                        4 => {
                            // add a Vec<i32> element to the list
                            // randomize vec size 1..10
                            let vec_size = rng.gen_range(1, 10);
                            let mut vec = vec![];

                            // randomize vec values
                            for _ in 1..vec_size {
                                vec.push(rng.gen::<i32>());
                            }

                            // add vec as an element to the list
                            assert!(db.ladd(&list_key, &vec).is_some());
                        }
                        _ => panic!("Cannot add list inside a list!"),
                    }
                }
            }
            _ => panic!("Chosen wrong type to generate!"),
        }
    }

    // verify that the number of values we wanted to generate equals to actually generated
    assert_eq!(map.iter().len(), generate_keys);

    // dump DB to file
    assert!(db.dump().is_ok());

    // read again from file
    let read_db = PickleDb::load_read_only(&db_name, ser_method!(ser_method_int)).unwrap();

    // iterate every key/value_type in map saved before
    for (key, val_type) in map.iter() {
        // verify key exists in db
        assert!(
            read_db.exists(key),
            "Key {} of type {} isn't found",
            key,
            val_type
        );

        // get the value according to the value_type saved
        match *val_type {
            "i32" => assert!(read_db.get::<i32>(key).is_some()),
            "f32" => assert!(read_db.get::<f32>(key).is_some()),
            "string" => assert!(read_db.get::<String>(key).is_some()),
            "vec" => assert!(read_db.get::<Vec<i32>>(key).is_some()),
            "list" => assert!(read_db.lexists(key)),
            _ => (),
        }
    }

    // check that the total number of keys in db equals to number of keys generated
    assert_eq!(read_db.total_keys(), generate_keys);
}
