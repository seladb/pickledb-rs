#![allow(clippy::float_cmp)]

use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use serde::{Deserialize, Serialize};

mod common;

#[cfg(test)]
extern crate rstest;

use rstest::rstest_parametrize;

#[allow(clippy::cognitive_complexity)]
#[rstest_parametrize(ser_method_int, case(0), case(1), case(2), case(3))]
fn basic_lists(ser_method_int: i32) {
    test_setup!("basic_lists", ser_method_int, db_name);

    let mut db = PickleDb::new(
        &db_name,
        PickleDbDumpPolicy::AutoDump,
        ser_method!(ser_method_int),
    );

    db.lcreate("list1").unwrap();

    // add a number to list1
    let num = 100;
    assert!(db.ladd("list1", &num).is_some());

    // add a floating point number to list1
    let float_num = 1.224;
    assert!(db.ladd("list1", &float_num).is_some());

    // add a string to list1
    let mystr = String::from("my string");
    assert!(db.ladd("list1", &mystr).is_some());

    // add a Vec to list1
    let myvec = vec![1, 2, 3];
    assert!(db.ladd("list1", &myvec).is_some());

    // add a struct to list1
    #[derive(Serialize, Deserialize, Debug)]
    struct Coor {
        x: i32,
        y: i32,
    }
    let mycoor = Coor { x: 1, y: 2 };
    assert!(db.ladd("list1", &mycoor).is_some());

    // create another list
    db.lcreate("list2").unwrap();

    // add a number to list2
    let num2 = 200;
    assert!(db.ladd("list2", &num2).is_some());

    // add a string to list2
    let mystr2 = String::from("hello world");
    assert!(db.ladd("list2", &mystr2).is_some());

    // read first item in list1 - int
    assert_eq!(db.lget::<i32>("list1", 0).unwrap(), num);

    // read fourth item in list1 - vec
    assert_eq!(db.lget::<Vec<i32>>("list1", 3).unwrap(), myvec);

    // read second item in list2 - string
    assert_eq!(db.lget::<String>("list2", 1).unwrap(), mystr2);

    // read second item in list1 - float
    assert_eq!(db.lget::<f32>("list1", 1).unwrap(), float_num);

    // read third item in list1 - string
    assert_eq!(db.lget::<String>("list1", 2).unwrap(), mystr);

    // read first item in list1 - int
    assert_eq!(db.lget::<i32>("list2", 0).unwrap(), num2);

    // read fifth item in list1 - Coor
    assert_eq!(db.lget::<Coor>("list1", 4).unwrap().x, mycoor.x);
    assert_eq!(db.lget::<Coor>("list1", 4).unwrap().y, mycoor.y);

    // verify lists length
    assert_eq!(db.llen("list1"), 5);
    assert_eq!(db.llen("list2"), 2);
    // list that doesn't exist
    assert_eq!(db.llen("list3"), 0);

    // load the file as read only db
    let read_db = PickleDb::load(
        &db_name,
        PickleDbDumpPolicy::NeverDump,
        ser_method!(ser_method_int),
    )
    .unwrap();

    // verify lists length
    assert_eq!(read_db.llen("list1"), 5);
    assert_eq!(read_db.llen("list2"), 2);

    // read first item in list1 - int
    assert_eq!(read_db.lget::<i32>("list1", 0).unwrap(), num);

    // read fourth item in list1 - vec
    assert_eq!(read_db.lget::<Vec<i32>>("list1", 3).unwrap(), myvec);

    // read second item in list2 - string
    assert_eq!(read_db.lget::<String>("list2", 1).unwrap(), mystr2);

    // read second item in list1 - float
    assert_eq!(read_db.lget::<f32>("list1", 1).unwrap(), float_num);

    // read third item in list1 - string
    assert_eq!(read_db.lget::<String>("list1", 2).unwrap(), mystr);

    // read first item in list1 - int
    assert_eq!(read_db.lget::<i32>("list2", 0).unwrap(), num2);

    // read fifth item in list1 - Coor
    assert_eq!(read_db.lget::<Coor>("list1", 4).unwrap().x, mycoor.x);
    assert_eq!(read_db.lget::<Coor>("list1", 4).unwrap().y, mycoor.y);
}

#[rstest_parametrize(ser_method_int, case(0), case(1), case(2), case(3))]
fn add_and_extend_lists(ser_method_int: i32) {
    test_setup!("add_and_extend_lists", ser_method_int, db_name);

    let mut db = PickleDb::new(
        &db_name,
        PickleDbDumpPolicy::AutoDump,
        ser_method!(ser_method_int),
    );

    // create 3 lists
    db.lcreate("list1").unwrap();
    db.lcreate("list2").unwrap();
    db.lcreate("list3").unwrap();

    // list1 - add 6 elements using lextend
    assert!(db.lextend("list1", &[1, 2, 3, 4, 5, 6]).is_some());

    // list1 - add 6 elements using ladd
    db.ladd("list2", &1)
        .unwrap()
        .ladd(&2)
        .ladd(&3)
        .ladd(&4)
        .ladd(&5)
        .ladd(&6);

    // list3 - add 6 elements using lextend and ladd
    db.ladd("list3", &1)
        .unwrap()
        .lextend(&[2, 3])
        .ladd(&4)
        .lextend(&[5, 6]);

    // verify lists length
    assert_eq!(db.llen("list1"), 6);
    assert_eq!(db.llen("list2"), 6);
    assert_eq!(db.llen("list3"), 6);

    // check all values in all lists
    for x in 0..5 {
        assert_eq!(db.lget::<i32>("list1", x as usize).unwrap(), x + 1);
        assert_eq!(db.lget::<i32>("list2", x as usize).unwrap(), x + 1);
        assert_eq!(db.lget::<i32>("list3", x as usize).unwrap(), x + 1);
    }

    // read db from file
    let read_db = PickleDb::load(
        &db_name,
        PickleDbDumpPolicy::NeverDump,
        ser_method!(ser_method_int),
    )
    .unwrap();

    // check all values in all lists
    for x in 0..5 {
        assert_eq!(read_db.lget::<i32>("list1", x as usize).unwrap(), x + 1);
        assert_eq!(read_db.lget::<i32>("list2", x as usize).unwrap(), x + 1);
        assert_eq!(read_db.lget::<i32>("list3", x as usize).unwrap(), x + 1);
    }
}

#[rstest_parametrize(ser_method_int, case(0), case(1), case(2), case(3))]
fn override_lists(ser_method_int: i32) {
    test_setup!("override_lists", ser_method_int, db_name);

    let mut db = PickleDb::new(
        &db_name,
        PickleDbDumpPolicy::AutoDump,
        ser_method!(ser_method_int),
    );

    // create a list and add some values to it
    db.lcreate("list1").unwrap().lextend(&["aa", "bb", "cc"]);

    // verify list len is 3
    assert_eq!(db.llen("list1"), 3);

    // override the list
    db.lcreate("list1").unwrap();

    // verify list is now empty (override)
    assert!(db.lexists("list1"));
    assert_eq!(db.llen("list1"), 0);

    // read the list from file and verify the same
    {
        let read_db = PickleDb::load(
            &db_name,
            PickleDbDumpPolicy::NeverDump,
            ser_method!(ser_method_int),
        )
        .unwrap();
        assert!(read_db.lexists("list1"));
        assert_eq!(read_db.llen("list1"), 0);
    }

    // add items to the override list
    assert!(db.lextend("list1", &[1, 2, 3, 4]).is_some());

    // verify list contains the new data
    assert!(db.lexists("list1"));
    assert_eq!(db.llen("list1"), 4);

    // read the list from file and verify the same
    {
        let read_db = PickleDb::load(
            &db_name,
            PickleDbDumpPolicy::NeverDump,
            ser_method!(ser_method_int),
        )
        .unwrap();
        assert!(read_db.lexists("list1"));
        assert_eq!(read_db.llen("list1"), 4);
    }
}

#[rstest_parametrize(ser_method_int, case(0), case(1), case(2), case(3))]
fn lget_corner_cases(ser_method_int: i32) {
    test_setup!("lget_corner_cases", ser_method_int, db_name);

    let mut db = PickleDb::new(
        &db_name,
        PickleDbDumpPolicy::DumpUponRequest,
        ser_method!(ser_method_int),
    );

    // create a list and add some values
    db.lcreate("list1")
        .unwrap()
        .lextend(&["hello", "world", "good", "morning"])
        .ladd(&100);

    // lget values that exist
    assert_eq!(db.lget::<String>("list1", 0).unwrap(), "hello");
    assert_eq!(db.lget::<i32>("list1", 4).unwrap(), 100);

    // lget values that exist but in the wrong type
    if let SerializationMethod::Bin = ser_method!(ser_method_int) {
        // N/A
    } else {
        assert!(db.lget::<i32>("list1", 0).is_none());
        assert!(db.lget::<Vec<i32>>("list1", 0).is_none());

        if let SerializationMethod::Yaml = ser_method!(ser_method_int) {
            // N/A
        } else {
            assert!(db.lget::<String>("list1", 4).is_none());
        }
    }

    // lget values out of bounds
    assert!(db.lget::<i32>("list1", 5).is_none());
    assert!(db.lget::<String>("list1", 5).is_none());

    // lget list that doesn't exist
    assert!(db.lget::<i32>("list2", 5).is_none());
}

#[rstest_parametrize(ser_method_int, case(0), case(1), case(2), case(3))]
fn add_to_non_existent_list(ser_method_int: i32) {
    test_setup!("lget_corner_cases", ser_method_int, db_name);

    let mut db = PickleDb::new(
        &db_name,
        PickleDbDumpPolicy::DumpUponRequest,
        ser_method!(ser_method_int),
    );

    let num = 100;
    let vec_of_nums = vec![1, 2, 3];

    // add items to list that doesn't exist
    assert!(db.ladd("list1", &num).is_none());
    assert!(db.lextend("list1", &vec_of_nums).is_none());

    // create a list
    db.lcreate("list1").unwrap();

    // add items to list that doesn't exist
    assert!(db.ladd("list2", &num).is_none());
    assert!(db.lextend("list2", &vec_of_nums).is_none());

    // add items to the list that was created
    assert!(db.ladd("list1", &num).is_some());
    assert!(db.lextend("list1", &vec_of_nums).is_some());

    // delete the list
    assert!(db.rem("list1").unwrap_or(false));

    // add items to list that doesn't exist
    assert!(db.ladd("list1", &num).is_none());
    assert!(db.lextend("list1", &vec_of_nums).is_none());
}

#[rstest_parametrize(ser_method_int, case(0), case(1), case(2), case(3))]
fn remove_list(ser_method_int: i32) {
    test_setup!("remove_list", ser_method_int, db_name);

    let mut db = PickleDb::new(
        &db_name,
        PickleDbDumpPolicy::AutoDump,
        ser_method!(ser_method_int),
    );

    // create some lists add add values to them
    db.lcreate("list1")
        .unwrap()
        .lextend(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

    db.lcreate("list2")
        .unwrap()
        .lextend(&['a', 'b', 'c', 'd', 'e']);

    db.lcreate("list3")
        .unwrap()
        .lextend(&[1.2, 1.3, 2.1, 3.1, 3.3, 7.889]);

    db.lcreate("list4")
        .unwrap()
        .lextend(&["aaa", "bbb", "ccc", "ddd", "eee"]);

    // verify number of lists in file
    {
        let read_db = PickleDb::load(
            &db_name,
            PickleDbDumpPolicy::NeverDump,
            ser_method!(ser_method_int),
        )
        .unwrap();
        assert_eq!(read_db.total_keys(), 4);
    }

    // remove list1 using rem
    assert!(db.rem("list1").unwrap_or(false));

    // verify number of lists
    assert_eq!(db.total_keys(), 3);

    // verify number of lists in file
    {
        let read_db = PickleDb::load(
            &db_name,
            PickleDbDumpPolicy::NeverDump,
            ser_method!(ser_method_int),
        )
        .unwrap();
        assert_eq!(read_db.total_keys(), 3);
    }

    // remove list1 using lrem_list
    assert_eq!(db.lrem_list("list3").unwrap_or(0), 6);

    // verify number of lists
    assert_eq!(db.total_keys(), 2);

    // verify number of lists in file
    {
        let read_db = PickleDb::load(
            &db_name,
            PickleDbDumpPolicy::NeverDump,
            ser_method!(ser_method_int),
        )
        .unwrap();
        assert_eq!(read_db.total_keys(), 2);
    }
}

#[rstest_parametrize(ser_method_int, case(0), case(1), case(2), case(3))]
fn remove_values_from_list(ser_method_int: i32) {
    test_setup!("remove_values_from_list", ser_method_int, db_name);

    let mut db = PickleDb::new(
        &db_name,
        PickleDbDumpPolicy::AutoDump,
        ser_method!(ser_method_int),
    );

    // add a struct to list1
    #[derive(Serialize, Deserialize, Debug)]
    struct MySquare {
        x: u32,
    }

    // create a list and add some values
    db.lcreate("list1")
        .unwrap()
        .lextend(&[1, 2, 3])
        .ladd(&String::from("hello"))
        .ladd(&1.234)
        .lextend(&[MySquare { x: 4 }, MySquare { x: 10 }]);

    // list now looks like this:
    // Indices: [0, 1, 2, 3,       4,     5,           6           ]
    // Values:  [1, 2, 3, "hello", 1.234, MySquare(4), MySquare(10)]

    // pop the floating number
    assert_eq!(db.lpop::<f64>("list1", 4).unwrap(), 1.234);

    // list now looks like this:
    // Indices: [0, 1, 2, 3,       4,           5           ]
    // Values:  [1, 2, 3, "hello", MySquare(4), MySquare(10)]

    assert_eq!(db.lget::<MySquare>("list1", 4).unwrap().x, 4);
    assert_eq!(db.lget::<String>("list1", 3).unwrap(), "hello");

    // read this from file as well
    {
        let read_db = PickleDb::load(
            &db_name,
            PickleDbDumpPolicy::NeverDump,
            ser_method!(ser_method_int),
        )
        .unwrap();
        assert_eq!(read_db.lget::<MySquare>("list1", 4).unwrap().x, 4);
        assert_eq!(read_db.lget::<String>("list1", 3).unwrap(), "hello");
    }

    // pop the first element
    assert_eq!(db.lpop::<i32>("list1", 0).unwrap(), 1);

    // list now looks like this:
    // Indices: [0, 1, 2,       3,           4           ]
    // Values:  [2, 3, "hello", MySquare(4), MySquare(10)]

    assert_eq!(db.lget::<MySquare>("list1", 4).unwrap().x, 10);
    assert_eq!(db.lget::<i32>("list1", 1).unwrap(), 3);

    // remove the "hello" string
    assert!(db
        .lrem_value("list1", &String::from("hello"))
        .unwrap_or(false));

    // list now looks like this:
    // Indices: [0, 1, 2,           3           ]
    // Values:  [2, 3, MySquare(4), MySquare(10)]

    assert_eq!(db.lget::<MySquare>("list1", 3).unwrap().x, 10);
    assert_eq!(db.lget::<i32>("list1", 1).unwrap(), 3);

    // read this from file as well
    {
        let read_db = PickleDb::load(
            &db_name,
            PickleDbDumpPolicy::NeverDump,
            ser_method!(ser_method_int),
        )
        .unwrap();
        assert_eq!(read_db.lget::<MySquare>("list1", 3).unwrap().x, 10);
        assert_eq!(read_db.lget::<i32>("list1", 1).unwrap(), 3);
    }

    // remove the MySquare(4)
    assert!(db.lrem_value("list1", &MySquare { x: 4 }).unwrap_or(false));

    // list now looks like this:
    // Indices: [0, 1, 2           ]
    // Values:  [2, 3, MySquare(10)]

    assert_eq!(db.lget::<MySquare>("list1", 2).unwrap().x, 10);
    assert_eq!(db.lget::<i32>("list1", 0).unwrap(), 2);

    // read this from file as well
    {
        let read_db = PickleDb::load(
            &db_name,
            PickleDbDumpPolicy::NeverDump,
            ser_method!(ser_method_int),
        )
        .unwrap();
        assert_eq!(read_db.lget::<MySquare>("list1", 2).unwrap().x, 10);
        assert_eq!(read_db.lget::<i32>("list1", 0).unwrap(), 2);
    }
}

#[rstest_parametrize(ser_method_int, case(0), case(1), case(2), case(3))]
fn list_with_special_strings(ser_method_int: i32) {
    test_setup!("list_with_special_strings", ser_method_int, db_name);

    let mut db = PickleDb::new(
        &db_name,
        PickleDbDumpPolicy::AutoDump,
        ser_method!(ser_method_int),
    );

    // create a list and add special strings to it
    db.lcreate("list1")
        .unwrap()
        .ladd(&String::from("\"double_quotes\""))
        .ladd(&String::from("\'single_quotes\'"))
        .ladd(&String::from("שָׁלוֹם"))
        .ladd(&String::from("😻"))
        .ladd(&String::from("\nescapes\t\r"))
        .ladd(&String::from("my\\folder"));

    // read special strings
    assert_eq!(
        db.lget::<String>("list1", 0).unwrap(),
        String::from("\"double_quotes\"")
    );
    assert_eq!(
        db.lget::<String>("list1", 1).unwrap(),
        String::from("\'single_quotes\'")
    );
    assert_eq!(db.lget::<String>("list1", 2).unwrap(), String::from("שָׁלוֹם"));
    assert_eq!(db.lget::<String>("list1", 3).unwrap(), String::from("😻"));
    assert_eq!(
        db.lget::<String>("list1", 4).unwrap(),
        String::from("\nescapes\t\r")
    );
    assert_eq!(
        db.lget::<String>("list1", 5).unwrap(),
        String::from("my\\folder")
    );

    // load db from file
    let read_db = PickleDb::load_read_only(&db_name, ser_method!(ser_method_int)).unwrap();

    // read strings from list loaded from file
    assert_eq!(
        read_db.lget::<String>("list1", 0).unwrap(),
        String::from("\"double_quotes\"")
    );
    assert_eq!(
        read_db.lget::<String>("list1", 1).unwrap(),
        String::from("\'single_quotes\'")
    );
    assert_eq!(
        read_db.lget::<String>("list1", 2).unwrap(),
        String::from("שָׁלוֹם")
    );
    assert_eq!(
        read_db.lget::<String>("list1", 3).unwrap(),
        String::from("😻")
    );
    assert_eq!(
        read_db.lget::<String>("list1", 4).unwrap(),
        String::from("\nescapes\t\r")
    );
    assert_eq!(
        read_db.lget::<String>("list1", 5).unwrap(),
        String::from("my\\folder")
    );
}

#[rstest_parametrize(ser_method_int, case(0), case(1), case(2), case(3))]
fn list_iter_test(ser_method_int: i32) {
    test_setup!("list_iter_test", ser_method_int, db_name);

    // create a db with auto_dump == true
    let mut db = PickleDb::new(
        &db_name,
        PickleDbDumpPolicy::AutoDump,
        ser_method!(ser_method_int),
    );

    let values = (
        1,
        1.1,
        String::from("value"),
        vec![1, 2, 3],
        ('a', 'b', 'c'),
    );

    // create a list with some values
    db.lcreate("list1")
        .unwrap()
        .ladd(&values.0)
        .ladd(&values.1)
        .ladd(&values.2)
        .ladd(&values.3)
        .ladd(&values.4);

    let mut index = 0;

    // iterate over the list
    for item in db.liter("list1") {
        // check each item
        match index {
            0 => assert_eq!(item.get_item::<i32>().unwrap(), values.0),
            1 => assert_eq!(item.get_item::<f32>().unwrap(), values.1),
            2 => assert_eq!(item.get_item::<String>().unwrap(), values.2),
            3 => assert_eq!(item.get_item::<Vec<i32>>().unwrap(), values.3),
            4 => assert_eq!(item.get_item::<(char, char, char)>().unwrap(), values.4),
            _ => panic!(),
        }
        index += 1;
    }

    // verify iterator went over all the items
    assert_eq!(index, 5);
}

#[allow(unused_attributes)]
#[should_panic]
#[rstest_parametrize(ser_method_int, case(0), case(1), case(2), case(3))]
fn list_doesnt_exist_iter_test(ser_method_int: i32) {
    test_setup!("list_doesnt_exist_iter_test", ser_method_int, db_name);

    // create a db with auto_dump == true
    let mut db = PickleDb::new(
        &db_name,
        PickleDbDumpPolicy::AutoDump,
        ser_method!(ser_method_int),
    );

    let values = (
        1,
        1.1,
        String::from("value"),
        vec![1, 2, 3],
        ('a', 'b', 'c'),
    );

    // create a list with some values
    db.lcreate("list1")
        .unwrap()
        .ladd(&values.0)
        .ladd(&values.1)
        .ladd(&values.2)
        .ladd(&values.3)
        .ladd(&values.4);

    // iterate over a non-existent list - should panic here
    for _item in db.liter("list2") {}
}
