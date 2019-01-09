use pickledb::PickleDb;

mod common;

#[macro_use]
extern crate serde_derive;

#[test]
fn basic_set_get() {
    set_test_rsc!("basic_set_get.db");

    let mut db = PickleDb::new("basic_set_get.db", false);

    // set a number
    let num = 100;
    db.set("num", &num);

    // set a floating point number
    let float_num = 1.224;
    db.set("float", &float_num);

    // set a String
    let mystr = String::from("my string");
    db.set("string", &mystr);

    // set a Vec
    let myvec = vec![1,2,3];
    db.set("vec", &myvec);

    // set a struct
    #[derive(Serialize, Deserialize, Debug)]
    struct Coor {
        x: i32,
        y: i32,
    }
    let mycoor = Coor { x: 1, y
    : 2 };
    db.set("struct", &mycoor);


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


#[test]
fn set_load_get() {
    set_test_rsc!("set_load_get.db");

    // create a db with auto_dump == false
    let mut db = PickleDb::new("set_load_get.db", false);

    // set a number
    let num = 100;
    db.set("num", &num);

    // set a floating point number
    let float_num = 1.224;
    db.set("float", &float_num);

    // set a String
    let mystr = String::from("my string");
    db.set("string", &mystr);

    // set a Vec
    let myvec = vec![1,2,3];
    db.set("vec", &myvec);

    // set a struct
    #[derive(Serialize, Deserialize, Debug)]
    struct Coor {
        x: i32,
        y: i32,
    }
    let mycoor = Coor { x: 1, y
    : 2 };
    db.set("struct", &mycoor);


    // dump db to file
    assert!(db.dump());

    // read db from file
    let read_db = PickleDb::load("set_load_get.db", false).unwrap();

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


#[test]
fn set_load_get_auto_dump() {
    set_test_rsc!("set_load_get_auto_dump.db");

    // create a db with auto_dump == true
    let mut db = PickleDb::new("set_load_get_auto_dump.db", true);

    // set a number
    let num = 100;
    db.set("num", &num);

    // set a floating point number
    let float_num = 1.224;
    db.set("float", &float_num);

    // set a String
    let mystr = String::from("my string");
    db.set("string", &mystr);

    // set a Vec
    let myvec = vec![1,2,3];
    db.set("vec", &myvec);

    // set a struct
    #[derive(Serialize, Deserialize, Debug)]
    struct Coor {
        x: i32,
        y: i32,
    }
    let mycoor = Coor { x: 1, y
    : 2 };
    db.set("struct", &mycoor);


    let read_db = PickleDb::load("set_load_get_auto_dump.db", false).unwrap();

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

#[test]
fn set_load_get_auto_dump2() {
    set_test_rsc!("set_load_get_auto_dump2.db");

    // create a db with auto_dump == true
    let mut db = PickleDb::new("set_load_get_auto_dump2.db", true);

    // set a number
    let num = 100;
    db.set("num", &num);

    // read this number immediately
    {
        let read_db = PickleDb::load("set_load_get_auto_dump2.db", false).unwrap();
        assert_eq!(read_db.get::<i32>("num").unwrap(), num);
    }

    // set another number
    let num2 = 200;
    db.set("num2", &num2);

    // read this other number immediately
    {
        let read_db = PickleDb::load("set_load_get_auto_dump2.db", false).unwrap();
        assert_eq!(read_db.get::<i32>("num2").unwrap(), num2);
    }

    // set a different value for a given key
    db.set("num", &101);

    // read the new value
    assert_eq!(db.get::<i32>("num").unwrap(), 101);
    {
        let read_db = PickleDb::load("set_load_get_auto_dump2.db", false).unwrap();
        assert_eq!(read_db.get::<i32>("num").unwrap(), 101);
    }

    // set a different value of a different type for a given key
    db.set("num", &vec![1,2,3]);

    // read the new value
    assert!(db.get::<i32>("num").is_none());
    assert_eq!(db.get::<Vec<i32>>("num").unwrap(), vec![1,2,3]);
    {
        let read_db = PickleDb::load("set_load_get_auto_dump2.db", false).unwrap();
        assert_eq!(read_db.get::<Vec<i32>>("num").unwrap(), vec![1,2,3]);
    }


}

#[test]
fn set_special_strings() {
    set_test_rsc!("set_special_strings.db");

    // create a db with auto_dump == true
    let mut db = PickleDb::new("set_special_strings.db", true);

    db.set("string1", &String::from("\"dobule_quotes\""));
    db.set("string2", &String::from("\'single_quotes\'"));
    db.set("string3", &String::from("שָׁלוֹם"));
    db.set("string4", &String::from("😻"));
    db.set("string5", &String::from("\nescapes\t\r"));
    db.set("string6", &String::from("my\\folder"));

    let read_db = PickleDb::load("set_special_strings.db", false).unwrap();
    assert_eq!(read_db.get::<String>("string1").unwrap(), String::from("\"dobule_quotes\""));
    assert_eq!(read_db.get::<String>("string2").unwrap(), String::from("\'single_quotes\'"));
    assert_eq!(read_db.get::<String>("string3").unwrap(), String::from("שָׁלוֹם"));
    assert_eq!(read_db.get::<String>("string4").unwrap(), String::from("😻"));
    assert_eq!(read_db.get::<String>("string5").unwrap(), String::from("\nescapes\t\r"));
    assert_eq!(read_db.get::<String>("string6").unwrap(), String::from("my\\folder"));
}

#[test]
fn edge_cases() {
    set_test_rsc!("edge_cases.db");

    // create a db with auto_dump == true
    let mut db = PickleDb::new("edge_cases.db", true);

    let x = 123;
    db.set("num", &x);

    // load a read only version of the db from file
    let read_db = PickleDb::load("edge_cases.db", false).unwrap();

    assert_eq!(db.get::<i32>("num"), Some(x));
    assert_eq!(read_db.get::<i32>("num"), Some(x));
    assert_eq!(db.get::<String>("num"), None);
    assert_eq!(read_db.get::<String>("num"), None);
}

#[test]
fn get_all_keys() {
    set_test_rsc!("get_all_keys.db");

    // create a db with auto_dump == true
    let mut db = PickleDb::new("get_all_keys.db", true);

    // insert 10 keys: key0..key9
    let num = 100;
    for i in 0..10 {
        db.set(&format!("{}{}", "key", i), &num);
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

#[test]
fn rem_keys() {
    set_test_rsc!("rem_keys.db");

    // create a db with auto_dump == true
    let mut db = PickleDb::new("rem_keys.db", true);

    // insert 10 keys: key0..key9
    let num = 100;
    for i in 0..10 {
        db.set(&format!("{}{}", "key", i), &num);
    }

    // remove 2 keys
    assert!(db.rem("key5"));
    assert!(db.rem("key8"));

    // verify we only have 8 keys now
    assert_eq!(db.total_keys(), 8);

    // verify both keys were removed
    for i in vec![5,8].iter() {
        assert_eq!(db.exists(&format!("{}{}", "key", i)), false);
    }

    // verify the other keys are still there
    for i in vec![0,1,2,3,4,6,7,9].iter() {
        assert!(db.exists(&format!("{}{}", "key", i)));
    }

    // verify keys were also removed from the file
    let read_db = PickleDb::load("rem_keys.db", false).unwrap();
    assert_eq!(read_db.total_keys(), 8);
}

