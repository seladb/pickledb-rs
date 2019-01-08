use pickledb::PickleDb;

mod common;

#[macro_use]
extern crate serde_derive;

#[test]
fn basic_lists() {
    set_test_rsc!("basic_lists.db");

    let mut db = PickleDb::new("basic_lists.db", true);

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
    let read_db = PickleDb::load("basic_lists.db", false).unwrap();

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

    let mut db = PickleDb::new("add_and_extend_lists.db", true);

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
    let read_db = PickleDb::load("add_and_extend_lists.db", false).unwrap();

    // check all values in all lists
    for x in 0..5 {
        assert_eq!(read_db.lget::<i32>("list1", x as usize).unwrap(), x+1);
        assert_eq!(read_db.lget::<i32>("list2", x as usize).unwrap(), x+1);
        assert_eq!(read_db.lget::<i32>("list3", x as usize).unwrap(), x+1);
    }
}

#[test]
fn lget_corner_cases() {
    set_test_rsc!("lget_corner_cases.db");

    let mut db = PickleDb::new("lget_corner_cases.db", false);

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

    let mut db = PickleDb::new("add_to_non_existent_list.db", false);

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
fn list_with_special_strings() {
    set_test_rsc!("list_with_special_strings.db");

    let mut db = PickleDb::new("list_with_special_strings.db", true);

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
    let read_db = PickleDb::load("list_with_special_strings.db", false).unwrap();

    // read strgins from list loaded from file
    assert_eq!(read_db.lget::<String>("list1", 0).unwrap(), String::from("\"dobule_quotes\""));
    assert_eq!(read_db.lget::<String>("list1", 1).unwrap(), String::from("\'single_quotes\'"));
    assert_eq!(read_db.lget::<String>("list1", 2).unwrap(), String::from("שָׁלוֹם"));
    assert_eq!(read_db.lget::<String>("list1", 3).unwrap(), String::from("😻"));
    assert_eq!(read_db.lget::<String>("list1", 4).unwrap(), String::from("\nescapes\t\r"));
    assert_eq!(read_db.lget::<String>("list1", 5).unwrap(), String::from("my\\folder"));
}