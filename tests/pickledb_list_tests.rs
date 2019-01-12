use pickledb::{PickleDb,PickleDbDumpPolicy};

mod common;

#[macro_use]
extern crate serde_derive;

#[test]
fn basic_lists() {
    set_test_rsc!("basic_lists.db");

    let mut db = PickleDb::new("basic_lists.db", PickleDbDumpPolicy::AutoDump);

    db.lcreate("list1");

    // add a number to list1
    let num = 100;
    assert!(db.ladd("list1", &num));

    // add a floating point number to list1
    let float_num = 1.224;
    assert!(db.ladd("list1", &float_num));

    // add a string to list1
    let mystr = String::from("my string");
    assert!(db.ladd("list1", &mystr));

    // add a Vec to list1
    let myvec = vec![1,2,3];
    assert!(db.ladd("list1", &myvec));

    // add a struct to list1
    #[derive(Serialize, Deserialize, Debug)]
    struct Coor {
        x: i32,
        y: i32,
    }
    let mycoor = Coor { x: 1, y
    : 2 };
    assert!(db.ladd("list1", &mycoor));

    // create another list
    db.lcreate("list2");

    // add a number to list2
    let num2 = 200;
    assert!(db.ladd("list2", &num2));

    // add a string to list2
    let mystr2 = String::from("hello world");
    assert!(db.ladd("list2", &mystr2));


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
    let read_db = PickleDb::load("basic_lists.db", PickleDbDumpPolicy::NeverDump).unwrap();

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

#[test]
fn add_and_extend_lists() {
    set_test_rsc!("add_and_extend_lists.db");

    let mut db = PickleDb::new("add_and_extend_lists.db", PickleDbDumpPolicy::AutoDump);

    // create 3 lists
    db.lcreate("list1");
    db.lcreate("list2");
    db.lcreate("list3");

    // list1 - add 6 elements using lextend
    assert!(db.lextend("list1", &vec![1,2,3,4,5,6]));

    // list1 - add 6 elements using ladd
    assert!(db.ladd("list2", &1));
    assert!(db.ladd("list2", &2));
    assert!(db.ladd("list2", &3));
    assert!(db.ladd("list2", &4));
    assert!(db.ladd("list2", &5));
    assert!(db.ladd("list2", &6));

    // list3 - add 6 elements using lextend and ladd
    assert!(db.ladd("list3", &1));
    assert!(db.lextend("list3", &vec![2,3]));
    assert!(db.ladd("list3", &4));
    assert!(db.lextend("list3", &vec![5,6]));

    // verify lists length
    assert_eq!(db.llen("list1"), 6);
    assert_eq!(db.llen("list2"), 6);
    assert_eq!(db.llen("list3"), 6);

    // check all values in all lists
    for x in 0..5 {
        assert_eq!(db.lget::<i32>("list1", x as usize).unwrap(), x+1);
        assert_eq!(db.lget::<i32>("list2", x as usize).unwrap(), x+1);
        assert_eq!(db.lget::<i32>("list3", x as usize).unwrap(), x+1);
    }

    // read db from file
    let read_db = PickleDb::load("add_and_extend_lists.db", PickleDbDumpPolicy::NeverDump).unwrap();

    // check all values in all lists
    for x in 0..5 {
        assert_eq!(read_db.lget::<i32>("list1", x as usize).unwrap(), x+1);
        assert_eq!(read_db.lget::<i32>("list2", x as usize).unwrap(), x+1);
        assert_eq!(read_db.lget::<i32>("list3", x as usize).unwrap(), x+1);
    }
}

#[test]
fn override_lists() {
    set_test_rsc!("override_lists.db");

    let mut db = PickleDb::new("override_lists.db", PickleDbDumpPolicy::AutoDump);

    // create a list and add some values to it
    db.lcreate("list1");
    assert!(db.lextend("list1", &vec!["aa", "bb", "cc"]));

    // verify list len is 3
    assert_eq!(db.llen("list1"), 3);

    // override the list
    db.lcreate("list1");

    // verify list is now empty (override)
    assert!(db.lexists("list1"));
    assert_eq!(db.llen("list1"), 0);

    // read the list from file and verify the same
    {
        let read_db = PickleDb::load("override_lists.db", PickleDbDumpPolicy::NeverDump).unwrap();
        assert!(read_db.lexists("list1"));
        assert_eq!(read_db.llen("list1"), 0);
    }

    // add items to the override list
    assert!(db.lextend("list1", &vec![1,2,3,4]));

    // verify list contains the new data
    assert!(db.lexists("list1"));
    assert_eq!(db.llen("list1"), 4);

    // read the list from file and verify the same
    {
        let read_db = PickleDb::load("override_lists.db", PickleDbDumpPolicy::NeverDump).unwrap();
        assert!(read_db.lexists("list1"));
        assert_eq!(read_db.llen("list1"), 4);
    }
}

#[test]
fn lget_corner_cases() {
    set_test_rsc!("lget_corner_cases.db");

    let mut db = PickleDb::new("lget_corner_cases.db", PickleDbDumpPolicy::DumpUponRequest);

    // create a list and add some values
    db.lcreate("list1");
    assert!(db.lextend("list1", &vec!["hello", "world", "good", "morning"]));
    assert!(db.ladd("list1", &100));

    // lget values that exist
    assert_eq!(db.lget::<String>("list1", 0).unwrap(), "hello");
    assert_eq!(db.lget::<i32>("list1", 4).unwrap(), 100);

    // lget values that exist but in the wrong type
    assert!(db.lget::<i32>("list1", 0).is_none());
    assert!(db.lget::<Vec<i32>>("list1", 0).is_none());
    assert!(db.lget::<String>("list1", 4).is_none());
    
    // lget values out of bounds
    assert!(db.lget::<i32>("list1", 5).is_none());
    assert!(db.lget::<String>("list1", 5).is_none());

    // lget list that doesn't exist
    assert!(db.lget::<i32>("list2", 5).is_none());
}

#[test]
fn add_to_non_existent_list() {
    set_test_rsc!("add_to_non_existent_list.db");

    let mut db = PickleDb::new("add_to_non_existent_list.db", PickleDbDumpPolicy::DumpUponRequest);

    let num = 100;
    let vec_of_nums = vec![1,2,3];

    // add items to list that doesn't exist
    assert_eq!(db.ladd("list1", &num), false);
    assert_eq!(db.lextend("list1", &vec_of_nums), false);

    // creat a list
    db.lcreate("list1");

    // add items to list that doesn't exist
    assert_eq!(db.ladd("list2", &num), false);
    assert_eq!(db.lextend("list2", &vec_of_nums), false);

    // add items to the list that was created
    assert!(db.ladd("list1", &num));
    assert!(db.lextend("list1", &vec_of_nums));

    // delete the list
    assert!(db.rem("list1"));

    // add items to list that doesn't exist
    assert_eq!(db.ladd("list1", &num), false);
    assert_eq!(db.lextend("list1", &vec_of_nums), false);
}

#[test]
fn remove_list() {
    set_test_rsc!("remove_list.db");

    let mut db = PickleDb::new("remove_list.db", PickleDbDumpPolicy::AutoDump);

    // create some lists
    db.lcreate("list1");
    db.lcreate("list2");
    db.lcreate("list3");
    db.lcreate("list4");

    // add values to lists
    assert!(db.lextend("list1", &vec![1,2,3,4,5,6,7,8,9,10]));
    assert!(db.lextend("list2", &vec!['a', 'b', 'c', 'd', 'e']));
    assert!(db.lextend("list3", &vec![1.2, 1.3, 2.1, 3.1, 3.3, 7.889]));
    assert!(db.lextend("list4", &vec!["aaa", "bbb", "ccc", "ddd", "eee"]));

    // verify number of lists in file
    {
        let read_db = PickleDb::load("remove_list.db", PickleDbDumpPolicy::NeverDump).unwrap();
        assert_eq!(read_db.total_keys(), 4);
    }

    // remove list1 using rem
    assert!(db.rem("list1"));

    // verify number of lists
    assert_eq!(db.total_keys(), 3);

    // verify number of lists in file
    {
        let read_db = PickleDb::load("remove_list.db", PickleDbDumpPolicy::NeverDump).unwrap();
        assert_eq!(read_db.total_keys(), 3);
    }


    // remove list1 using lrem_list
    assert_eq!(db.lrem_list("list3"), 6);

    // verify number of lists
    assert_eq!(db.total_keys(), 2);

    // verify number of lists in file
    {
        let read_db = PickleDb::load("remove_list.db", PickleDbDumpPolicy::NeverDump).unwrap();
        assert_eq!(read_db.total_keys(), 2);
    }
}

#[test]
fn remove_values_from_list() {
    set_test_rsc!("remove_values_from_list.db");

    let mut db = PickleDb::new("remove_values_from_list.db", PickleDbDumpPolicy::AutoDump);

    // add a struct to list1
    #[derive(Serialize, Deserialize, Debug)]
    struct MySquare {
        x: u32,
    }

    // create a list and add some values
    db.lcreate("list1");
    assert!(db.lextend("list1", &vec![1,2,3]));
    assert!(db.ladd("list1", &String::from("hello")));
    assert!(db.ladd("list1", &1.234));
    assert!(db.lextend("list1", &vec![MySquare { x: 4 }, MySquare { x: 10 }]));

    // list now looks like this:
    // Indices: [0, 1, 2, 3,       4,     5,           6           ]
    // Values:  [1, 2, 3, "hello", 1.234, MySquare(4), MySquare(10)]

    // pop the floating number
    assert_eq!(db.lpop::<f32>("list1", 4).unwrap(), 1.234);

    // list now looks like this:
    // Indices: [0, 1, 2, 3,       4,           5           ]
    // Values:  [1, 2, 3, "hello", MySquare(4), MySquare(10)]

    assert_eq!(db.lget::<MySquare>("list1", 4).unwrap().x, 4);
    assert_eq!(db.lget::<String>("list1", 3).unwrap(), "hello");

    // read this from file as well
    {
        let read_db = PickleDb::load("remove_values_from_list.db", PickleDbDumpPolicy::NeverDump).unwrap();
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
    assert!(db.lrem_value("list1", &String::from("hello")));

    // list now looks like this:
    // Indices: [0, 1, 2,           3           ]
    // Values:  [2, 3, MySquare(4), MySquare(10)]

    assert_eq!(db.lget::<MySquare>("list1", 3).unwrap().x, 10);
    assert_eq!(db.lget::<i32>("list1", 1).unwrap(), 3);

    // read this from file as well
    {
        let read_db = PickleDb::load("remove_values_from_list.db", PickleDbDumpPolicy::NeverDump).unwrap();
        assert_eq!(read_db.lget::<MySquare>("list1", 3).unwrap().x, 10);
        assert_eq!(read_db.lget::<i32>("list1", 1).unwrap(), 3);
    }

    // remove the MySquare(4)
    assert!(db.lrem_value("list1", &MySquare { x: 4 }));

    // list now looks like this:
    // Indices: [0, 1, 2           ]
    // Values:  [2, 3, MySquare(10)]

    assert_eq!(db.lget::<MySquare>("list1", 2).unwrap().x, 10);
    assert_eq!(db.lget::<i32>("list1", 0).unwrap(), 2);

    // read this from file as well
    {
        let read_db = PickleDb::load("remove_values_from_list.db", PickleDbDumpPolicy::NeverDump).unwrap();
        assert_eq!(read_db.lget::<MySquare>("list1", 2).unwrap().x, 10);
        assert_eq!(read_db.lget::<i32>("list1", 0).unwrap(), 2);
    }
}

#[test]
fn list_with_special_strings() {
    set_test_rsc!("list_with_special_strings.db");

    let mut db = PickleDb::new("list_with_special_strings.db", PickleDbDumpPolicy::AutoDump);

    // create a list
    db.lcreate("list1");

    // add special strings to the list
    assert!(db.ladd("list1", &String::from("\"dobule_quotes\"")));
    assert!(db.ladd("list1", &String::from("\'single_quotes\'")));
    assert!(db.ladd("list1", &String::from("שָׁלוֹם")));
    assert!(db.ladd("list1", &String::from("😻")));
    assert!(db.ladd("list1", &String::from("\nescapes\t\r")));
    assert!(db.ladd("list1", &String::from("my\\folder")));

    // read special strings
    assert_eq!(db.lget::<String>("list1", 0).unwrap(), String::from("\"dobule_quotes\""));
    assert_eq!(db.lget::<String>("list1", 1).unwrap(), String::from("\'single_quotes\'"));
    assert_eq!(db.lget::<String>("list1", 2).unwrap(), String::from("שָׁלוֹם"));
    assert_eq!(db.lget::<String>("list1", 3).unwrap(), String::from("😻"));
    assert_eq!(db.lget::<String>("list1", 4).unwrap(), String::from("\nescapes\t\r"));
    assert_eq!(db.lget::<String>("list1", 5).unwrap(), String::from("my\\folder"));

    // load db from file
    let read_db = PickleDb::load_read_only("list_with_special_strings.db").unwrap();

    // read strgins from list loaded from file
    assert_eq!(read_db.lget::<String>("list1", 0).unwrap(), String::from("\"dobule_quotes\""));
    assert_eq!(read_db.lget::<String>("list1", 1).unwrap(), String::from("\'single_quotes\'"));
    assert_eq!(read_db.lget::<String>("list1", 2).unwrap(), String::from("שָׁלוֹם"));
    assert_eq!(read_db.lget::<String>("list1", 3).unwrap(), String::from("😻"));
    assert_eq!(read_db.lget::<String>("list1", 4).unwrap(), String::from("\nescapes\t\r"));
    assert_eq!(read_db.lget::<String>("list1", 5).unwrap(), String::from("my\\folder"));
}