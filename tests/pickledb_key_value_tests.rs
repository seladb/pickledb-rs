#![allow(clippy::float_cmp)]

use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use serde::{Deserialize, Serialize};

mod common;

#[cfg(test)]
extern crate rstest;

use rstest::rstest_parametrize;

#[rstest_parametrize(ser_method_int, case(0), case(1), case(2), case(3))]
fn basic_set_get(ser_method_int: i32) {
    test_setup!("basic_set_get", ser_method_int, db_name);

    let mut db = PickleDb::new(
        &db_name,
        PickleDbDumpPolicy::AutoDump,
        ser_method!(ser_method_int),
    );

    // set a number
    let num = 100;
    db.set("num", &num).unwrap();

    // set a floating point number
    let float_num = 1.224;
    db.set("float", &float_num).unwrap();

    // set a String
    let mystr = String::from("my string");
    db.set("string", &mystr).unwrap();

    // set a Vec
    let myvec = vec![1, 2, 3];
    db.set("vec", &myvec).unwrap();

    // set a struct
    #[derive(Serialize, Deserialize, Debug)]
    struct Coor {
        x: i32,
        y: i32,
    }
    let mycoor = Coor { x: 1, y: 2 };
    db.set("struct", &mycoor).unwrap();

    // read a num
    assert_eq!(db.get::<i32>("num").unwrap(), num);
    // read a floating point number
    assert_eq!(db.get::<f32>("float").unwrap(), float_num);
    // read a String
    assert_eq!(db.get::<String>("string").unwrap(), mystr);
    // read a Vec
    assert_eq!(db.get::<Vec<i32>>("vec").unwrap(), myvec);
    // read a struct
    assert_eq!(db.get::<Coor>("struct").unwrap().x, mycoor.x);
    assert_eq!(db.get::<Coor>("struct").unwrap().y, mycoor.y);
}

#[rstest_parametrize(ser_method_int, case(0), case(1), case(2), case(3))]
fn set_load_get(ser_method_int: i32) {
    test_setup!("set_load_get", ser_method_int, db_name);

    // create a db with auto_dump == false
    let mut db = PickleDb::new(
        &db_name,
        PickleDbDumpPolicy::DumpUponRequest,
        ser_method!(ser_method_int),
    );

    // set a number
    let num = 100;
    db.set("num", &num).unwrap();

    // set a floating point number
    let float_num = 1.224;
    db.set("float", &float_num).unwrap();

    // set a String
    let mystr = String::from("my string");
    db.set("string", &mystr).unwrap();

    // set a Vec
    let myvec = vec![1, 2, 3];
    db.set("vec", &myvec).unwrap();

    // set a struct
    #[derive(Serialize, Deserialize, Debug)]
    struct Coor {
        x: i32,
        y: i32,
    }
    let mycoor = Coor { x: 1, y: 2 };
    db.set("struct", &mycoor).unwrap();

    // dump db to file
    assert!(db.dump().is_ok());

    // read db from file
    let read_db = PickleDb::load_read_only(&db_name, ser_method!(ser_method_int)).unwrap();

    // read a num
    assert_eq!(read_db.get::<i32>("num").unwrap(), num);
    // read a floating point number
    assert_eq!(read_db.get::<f32>("float").unwrap(), float_num);
    // read a String
    assert_eq!(read_db.get::<String>("string").unwrap(), mystr);
    // read a Vec
    assert_eq!(read_db.get::<Vec<i32>>("vec").unwrap(), myvec);
    // read a struct
    assert_eq!(read_db.get::<Coor>("struct").unwrap().x, mycoor.x);
    assert_eq!(read_db.get::<Coor>("struct").unwrap().y, mycoor.y);
}

#[rstest_parametrize(ser_method_int, case(0), case(1), case(2), case(3))]
fn set_load_get_auto_dump(ser_method_int: i32) {
    test_setup!("set_load_get_auto_dump", ser_method_int, db_name);

    // create a db with auto_dump == true
    let mut db = PickleDb::new(
        &db_name,
        PickleDbDumpPolicy::AutoDump,
        ser_method!(ser_method_int),
    );

    // set a number
    let num = 100;
    db.set("num", &num).unwrap();

    // set a floating point number
    let float_num = 1.224;
    db.set("float", &float_num).unwrap();

    // set a String
    let mystr = String::from("my string");
    db.set("string", &mystr).unwrap();

    // set a Vec
    let myvec = vec![1, 2, 3];
    db.set("vec", &myvec).unwrap();

    // set a struct
    #[derive(Serialize, Deserialize, Debug)]
    struct Coor {
        x: i32,
        y: i32,
    }
    let mycoor = Coor { x: 1, y: 2 };
    db.set("struct", &mycoor).unwrap();

    let read_db = PickleDb::load_read_only(&db_name, ser_method!(ser_method_int)).unwrap();

    // read a num
    assert_eq!(read_db.get::<i32>("num").unwrap(), num);
    // read a floating point number
    assert_eq!(read_db.get::<f32>("float").unwrap(), float_num);
    // read a String
    assert_eq!(read_db.get::<String>("string").unwrap(), mystr);
    // read a Vec
    assert_eq!(read_db.get::<Vec<i32>>("vec").unwrap(), myvec);
    // read a struct
    assert_eq!(read_db.get::<Coor>("struct").unwrap().x, mycoor.x);
    assert_eq!(read_db.get::<Coor>("struct").unwrap().y, mycoor.y);
}

#[rstest_parametrize(ser_method_int, case(0), case(1), case(2), case(3))]
fn set_load_get_auto_dump2(ser_method_int: i32) {
    test_setup!("set_load_get_auto_dump2", ser_method_int, db_name);

    // create a db with auto_dump == true
    let mut db = PickleDb::new(
        &db_name,
        PickleDbDumpPolicy::AutoDump,
        ser_method!(ser_method_int),
    );

    // set a number
    let num = 100;
    db.set("num", &num).unwrap();

    // read this number immediately
    {
        let read_db = PickleDb::load_read_only(&db_name, ser_method!(ser_method_int)).unwrap();
        assert_eq!(read_db.get::<i32>("num").unwrap(), num);
    }

    // set another number
    let num2 = 200;
    db.set("num2", &num2).unwrap();

    // read this other number immediately
    {
        let read_db = PickleDb::load_read_only(&db_name, ser_method!(ser_method_int)).unwrap();
        assert_eq!(read_db.get::<i32>("num2").unwrap(), num2);
    }

    // set a different value for a given key
    db.set("num", &101).unwrap();

    // read the new value
    assert_eq!(db.get::<i32>("num").unwrap(), 101);
    {
        let read_db = PickleDb::load_read_only(&db_name, ser_method!(ser_method_int)).unwrap();
        assert_eq!(read_db.get::<i32>("num").unwrap(), 101);
    }

    // set a different value of a different type for a given key
    db.set("num", &vec![1, 2, 3]).unwrap();

    // read the new value
    if let SerializationMethod::Bin = ser_method!(ser_method_int) {
        // N/A
    } else {
        assert!(db.get::<i32>("num").is_none());
    }
    assert_eq!(db.get::<Vec<i32>>("num").unwrap(), vec![1, 2, 3]);
    {
        let read_db = PickleDb::load_read_only(&db_name, ser_method!(ser_method_int)).unwrap();
        assert_eq!(read_db.get::<Vec<i32>>("num").unwrap(), vec![1, 2, 3]);
    }
}

#[rstest_parametrize(ser_method_int, case(0), case(1), case(2), case(3))]
fn set_special_strings(ser_method_int: i32) {
    test_setup!("set_special_strings", ser_method_int, db_name);

    // create a db with auto_dump == true
    let mut db = PickleDb::new(
        &db_name,
        PickleDbDumpPolicy::AutoDump,
        ser_method!(ser_method_int),
    );

    db.set("string1", &String::from("\"double_quotes\""))
        .unwrap();
    db.set("string2", &String::from("\'single_quotes\'"))
        .unwrap();
    db.set("string3", &String::from("שָׁלוֹם")).unwrap();
    db.set("string4", &String::from("😻")).unwrap();
    db.set("string5", &String::from("\nescapes\t\r")).unwrap();
    db.set("string6", &String::from("my\\folder")).unwrap();

    let read_db = PickleDb::load_read_only(&db_name, ser_method!(ser_method_int)).unwrap();
    assert_eq!(
        read_db.get::<String>("string1").unwrap(),
        String::from("\"double_quotes\"")
    );
    assert_eq!(
        read_db.get::<String>("string2").unwrap(),
        String::from("\'single_quotes\'")
    );
    assert_eq!(
        read_db.get::<String>("string3").unwrap(),
        String::from("שָׁלוֹם")
    );
    assert_eq!(
        read_db.get::<String>("string4").unwrap(),
        String::from("😻")
    );
    assert_eq!(
        read_db.get::<String>("string5").unwrap(),
        String::from("\nescapes\t\r")
    );
    assert_eq!(
        read_db.get::<String>("string6").unwrap(),
        String::from("my\\folder")
    );
}

#[rstest_parametrize(ser_method_int, case(0), case(1), case(2), case(3))]
fn edge_cases(ser_method_int: i32) {
    test_setup!("edge_cases", ser_method_int, db_name);

    // create a db with auto_dump == true
    let mut db = PickleDb::new(
        &db_name,
        PickleDbDumpPolicy::AutoDump,
        ser_method!(ser_method_int),
    );

    let x = 123;
    db.set("num", &x).unwrap();

    // load a read only version of the db from file
    let read_db = PickleDb::load_read_only(&db_name, ser_method!(ser_method_int)).unwrap();

    assert_eq!(db.get::<i32>("num"), Some(x));
    assert_eq!(read_db.get::<i32>("num"), Some(x));
    if let SerializationMethod::Yaml = ser_method!(ser_method_int) {
        // N/A
    } else {
        assert_eq!(db.get::<String>("num"), None);
        assert_eq!(read_db.get::<String>("num"), None);
    }
}

#[rstest_parametrize(ser_method_int, case(0), case(1), case(2), case(3))]
fn get_all_keys(ser_method_int: i32) {
    test_setup!("get_all_keys", ser_method_int, db_name);

    // create a db with auto_dump == true
    let mut db = PickleDb::new(
        &db_name,
        PickleDbDumpPolicy::AutoDump,
        ser_method!(ser_method_int),
    );

    // insert 10 keys: key0..key9
    let num = 100;
    for i in 0..10 {
        db.set(&format!("{}{}", "key", i), &num).unwrap();
    }

    // verify we have 10 keys
    assert_eq!(db.total_keys(), 10);

    // get all keys
    let keys = db.get_all();

    // verify we got 10 keys
    assert_eq!(keys.len(), 10);

    // verify all key names are there
    for i in 0..9 {
        assert!(keys.iter().any(|key| key == &format!("{}{}", "key", i)));
    }
}

#[rstest_parametrize(ser_method_int, case(0), case(1), case(2), case(3))]
fn rem_keys(ser_method_int: i32) {
    test_setup!("rem_keys", ser_method_int, db_name);

    // create a db with auto_dump == true
    let mut db = PickleDb::new(
        &db_name,
        PickleDbDumpPolicy::AutoDump,
        ser_method!(ser_method_int),
    );

    // insert 10 keys: key0..key9
    let num = 100;
    for i in 0..10 {
        db.set(&format!("{}{}", "key", i), &num).unwrap();
    }

    // remove 2 keys
    assert!(db.rem("key5").unwrap_or(false));
    assert!(db.rem("key8").unwrap_or(false));

    // verify we only have 8 keys now
    assert_eq!(db.total_keys(), 8);

    // verify both keys were removed
    for i in vec![5, 8].iter() {
        assert!(!db.exists(&format!("{}{}", "key", i)));
    }

    // verify the other keys are still there
    for i in vec![0, 1, 2, 3, 4, 6, 7, 9].iter() {
        assert!(db.exists(&format!("{}{}", "key", i)));
    }

    // verify keys were also removed from the file
    let read_db = PickleDb::load_read_only(&db_name, ser_method!(ser_method_int)).unwrap();
    assert_eq!(read_db.total_keys(), 8);
}

#[rstest_parametrize(ser_method_int, case(0), case(1), case(2), case(3))]
fn iter_test(ser_method_int: i32) {
    test_setup!("iter_test", ser_method_int, db_name);

    // create a db with auto_dump == true
    let mut db = PickleDb::new(
        &db_name,
        PickleDbDumpPolicy::AutoDump,
        ser_method!(ser_method_int),
    );

    let keys = vec!["key1", "key2", "key3", "key4", "key5"];
    // add a few keys and values
    db.set(keys[0], &1).unwrap();
    db.set(keys[1], &1.1).unwrap();
    db.set(keys[2], &String::from("value1")).unwrap();
    db.set(keys[3], &vec![1, 2, 3]).unwrap();
    db.set(keys[4], &('a', 'b', 'c')).unwrap();

    // iterate the db
    let mut keys_seen = vec![false, false, false, false, false];
    for key_value in db.iter() {
        // find the index of the current key in the keys vec
        let index = keys.iter().position(|&k| k == key_value.get_key()).unwrap();

        // mark the key as seen
        keys_seen[index] = true;

        // verify the value
        match key_value.get_key() {
            "key1" => assert_eq!(key_value.get_value::<i32>().unwrap(), 1),
            "key2" => assert_eq!(key_value.get_value::<f64>().unwrap(), 1.1),
            "key3" => assert_eq!(
                key_value.get_value::<String>().unwrap(),
                String::from("value1")
            ),
            "key4" => assert_eq!(key_value.get_value::<Vec<i32>>().unwrap(), vec![1, 2, 3]),
            "key5" => assert_eq!(
                key_value.get_value::<(char, char, char)>().unwrap(),
                ('a', 'b', 'c')
            ),
            _ => panic!(),
        }
    }

    // verify all 5 keys were seen
    assert_eq!(keys_seen.iter().filter(|&t| *t).count(), 5);
}
