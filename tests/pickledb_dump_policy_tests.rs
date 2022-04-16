use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use std::time::Duration;
use std::{thread, time};

mod common;

#[cfg(test)]
extern crate rstest;

use rstest::rstest_parametrize;

#[rstest_parametrize(ser_method_int, case(0), case(1), case(2), case(3))]
fn auto_dump_policy_test(ser_method_int: i32) {
    test_setup!("auto_dump_policy_test", ser_method_int, db_name);

    // create a DB with AutoDump policy
    let mut db = PickleDb::new(
        &db_name,
        PickleDbDumpPolicy::AutoDump,
        ser_method!(ser_method_int),
    );

    // set a key-value pair
    assert!(db.set("key1", &1).is_ok());

    // verify the change in the DB
    {
        let read_db = PickleDb::load_read_only(&db_name, ser_method!(ser_method_int)).unwrap();
        assert_eq!(read_db.get::<i32>("key1").unwrap(), 1);
    }

    // remove a key
    assert!(db.rem("key1").unwrap_or(false));

    // verify the change in the DB
    {
        let read_db = PickleDb::load_read_only(&db_name, ser_method!(ser_method_int)).unwrap();
        assert!(read_db.get::<i32>("key1").is_none());
    }

    // create a list
    assert!(db.lcreate("list1").is_ok());

    // verify the change in the DB
    {
        let read_db = PickleDb::load_read_only(&db_name, ser_method!(ser_method_int)).unwrap();
        assert!(read_db.exists("list1"));
        assert_eq!(read_db.llen("list1"), 0);
    }

    // add values to list
    db.lextend("list1", &[1, 2, 3]);

    // verify the change in the DB
    {
        let read_db = PickleDb::load_read_only(&db_name, ser_method!(ser_method_int)).unwrap();
        assert_eq!(read_db.llen("list1"), 3);
    }

    // pop an item from a list
    db.lpop::<i32>("list1", 0);

    // verify the change in the DB
    {
        let read_db = PickleDb::load_read_only(&db_name, ser_method!(ser_method_int)).unwrap();
        assert_eq!(read_db.llen("list1"), 2);
    }

    // remove an item from a list
    assert!(db.lrem_value("list1", &2).unwrap_or(false));

    // verify the change in the DB
    {
        let read_db = PickleDb::load_read_only(&db_name, ser_method!(ser_method_int)).unwrap();
        assert_eq!(read_db.llen("list1"), 1);
    }

    // remove a list
    db.lrem_list("list1").unwrap();

    // verify the change in the DB
    {
        let read_db = PickleDb::load_read_only(&db_name, ser_method!(ser_method_int)).unwrap();
        assert!(!read_db.exists("list1"));
    }
}

#[rstest_parametrize(ser_method_int, case(0), case(1), case(2), case(3))]
fn read_only_policy_test(ser_method_int: i32) {
    test_setup!("read_only_policy_test", ser_method_int, db_name);

    // create a DB and set a value
    let mut db = PickleDb::new(
        &db_name,
        PickleDbDumpPolicy::AutoDump,
        ser_method!(ser_method_int),
    );
    assert!(db.set("key1", &String::from("value1")).is_ok());

    // create a read only instance of the same DB
    let mut read_db1 = PickleDb::load_read_only(&db_name, ser_method!(ser_method_int)).unwrap();

    // set a key-value pair in the read-only DB
    assert!(read_db1.set("key2", &String::from("value2")).is_ok());
    assert!(read_db1.exists("key2"));

    // verify the change isn't dumped to the file
    {
        let read_db2 = PickleDb::load_read_only(&db_name, ser_method!(ser_method_int)).unwrap();
        assert!(read_db2.exists("key1"));
        assert!(!read_db2.exists("key2"));
    }

    // try to dump data to the file
    assert!(read_db1.dump().is_ok());

    // verify the change isn't dumped to the file
    {
        let read_db2 = PickleDb::load_read_only(&db_name, ser_method!(ser_method_int)).unwrap();
        assert!(read_db2.exists("key1"));
        assert!(!read_db2.exists("key2"));
    }

    // drop the DB
    drop(read_db1);

    // verify the change isn't dumped to the file
    {
        let read_db2 = PickleDb::load_read_only(&db_name, ser_method!(ser_method_int)).unwrap();
        assert!(read_db2.exists("key1"));
        assert!(!read_db2.exists("key2"));
    }
}

#[rstest_parametrize(ser_method_int, case(0), case(1), case(2), case(3))]
fn dump_upon_request_policy_test(ser_method_int: i32) {
    test_setup!("dump_upon_request_policy_test", ser_method_int, db_name);

    // create a DB and set a value
    let mut db = PickleDb::new(
        &db_name,
        PickleDbDumpPolicy::DumpUponRequest,
        ser_method!(ser_method_int),
    );
    assert!(db.set("key1", &String::from("value1")).is_ok());

    // verify file is not yet created
    assert!(PickleDb::load_read_only(&db_name, ser_method!(ser_method_int)).is_err());

    // dump to file
    assert!(db.dump().is_ok());

    // verify the change is dumped to the file
    {
        let read_db = PickleDb::load_read_only(&db_name, ser_method!(ser_method_int)).unwrap();
        assert!(read_db.exists("key1"));
    }

    // set another key
    assert!(db.set("key2", &String::from("value2")).is_ok());

    // drop DB object
    drop(db);

    // verify the change is not dumped to the file
    {
        let read_db = PickleDb::load_read_only(&db_name, ser_method!(ser_method_int)).unwrap();
        assert!(read_db.exists("key1"));
        assert!(!read_db.exists("key2"));
    }
}

#[rstest_parametrize(ser_method_int, case(0), case(1), case(2), case(3))]
fn periodic_dump_policy_test(ser_method_int: i32) {
    test_setup!("periodic_dump_policy_test", ser_method_int, db_name);

    // create a DB and set a value
    let mut db = PickleDb::new(
        &db_name,
        PickleDbDumpPolicy::PeriodicDump(Duration::new(1, 0)),
        ser_method!(ser_method_int),
    );
    assert!(db.set("key1", &String::from("value1")).is_ok());

    // verify file is not yet created
    assert!(PickleDb::load_read_only(&db_name, ser_method!(ser_method_int)).is_err());

    // sleep for 0.5 sec
    thread::sleep(time::Duration::from_millis(500));

    // verify file is not yet created
    assert!(PickleDb::load_read_only(&db_name, ser_method!(ser_method_int)).is_err());

    // sleep for 0.55 sec
    thread::sleep(time::Duration::from_millis(550));

    // make another change in the DB
    assert!(db.set("key2", &String::from("value2")).is_ok());

    // verify the change is dumped to the file
    {
        let read_db = PickleDb::load_read_only(&db_name, ser_method!(ser_method_int)).unwrap();
        assert!(read_db.exists("key1"));
        assert!(read_db.exists("key2"));
    }

    // make another change in the DB
    assert!(db.set("key3", &String::from("value3")).is_ok());

    // verify the change is not yet dumped to the file
    {
        let read_db = PickleDb::load_read_only(&db_name, ser_method!(ser_method_int)).unwrap();
        assert!(!read_db.exists("key3"));
    }

    // dumb DB to file
    assert!(db.dump().is_ok());

    // verify the change is now dumped to the file
    {
        let read_db = PickleDb::load_read_only(&db_name, ser_method!(ser_method_int)).unwrap();
        assert!(read_db.exists("key3"));
    }

    // sleep for 1 more second
    thread::sleep(time::Duration::from_secs(1));

    // make another change in the DB
    assert!(db.set("key4", &String::from("value4")).is_ok());

    // verify the change is dumped to the file
    {
        let read_db = PickleDb::load_read_only(&db_name, ser_method!(ser_method_int)).unwrap();
        assert!(read_db.exists("key4"));
    }

    // make another change in the DB
    assert!(db.set("key5", &String::from("value5")).is_ok());

    // drop DB and verify change is written to DB
    drop(db);

    // verify the change is dumped to the file
    {
        let read_db = PickleDb::load_read_only(&db_name, ser_method!(ser_method_int)).unwrap();
        assert!(read_db.exists("key5"));
    }
}
