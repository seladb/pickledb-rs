use pickledb::error::ErrorType;
use pickledb::{PickleDb, PickleDbDumpPolicy};

#[macro_use(matches)]
extern crate matches;
extern crate fs2;

use fs2::FileExt;
use std::fs::File;

mod common;

#[test]
fn load_serialization_error_test() {
    set_test_rsc!("json_db.db");

    // create a new DB with Json serialization
    let mut db = PickleDb::new_json("json_db.db", PickleDbDumpPolicy::AutoDump);

    // set some values
    db.set("num", &100).unwrap();
    db.set("float", &1.1).unwrap();
    db.set("string", &String::from("my string")).unwrap();
    db.set("vec", &vec![1, 2, 3]).unwrap();

    // try to load the same DB with Bincode serialization, should fail
    let load_as_bin = PickleDb::load_bin("json_db.db", PickleDbDumpPolicy::NeverDump);
    assert!(load_as_bin.is_err());
    let load_as_bin_err = load_as_bin.err().unwrap();
    assert!(matches!(
        load_as_bin_err.get_type(),
        ErrorType::Serialization
    ));
    assert_eq!(load_as_bin_err.to_string(), "Cannot deserialize DB");

    // try to load the same DB with CBOR serialization, should fail
    let load_as_cbor = PickleDb::load_cbor("json_db.db", PickleDbDumpPolicy::NeverDump);
    assert!(load_as_cbor.is_err());
    let load_as_cbor_err = load_as_cbor.err().unwrap();
    assert!(matches!(
        load_as_cbor_err.get_type(),
        ErrorType::Serialization
    ));
    assert_eq!(load_as_cbor_err.to_string(), "Cannot deserialize DB");

    // try to load the same DB with Yaml serialization, should not fail because YAML is
    // a superset oj JSON
    assert!(PickleDb::load_yaml("json_db.db", PickleDbDumpPolicy::NeverDump).is_ok());
}

#[test]
fn load_error_test() {
    // load a file that doesn't exist and make sure we get an IO error
    let load_result = PickleDb::load_bin("doesnt_exists.db", PickleDbDumpPolicy::NeverDump);
    assert!(load_result.is_err());
    let load_result_err = load_result.err().unwrap();
    assert!(matches!(load_result_err.get_type(), ErrorType::Io));
}

#[allow(clippy::cognitive_complexity)]
#[test]
fn dump_error_test() {
    set_test_rsc!("dump_error_test.db");

    // I didn't find a way to effectively lock a file for writing on OS's
    // other than Windows. So until I find a solution I'm bypassing the test
    // for non-Windows OS's
    if cfg!(not(target_os = "windows")) {
        return;
    }

    // create a new DB with Json serialization
    let mut db = PickleDb::new_json("dump_error_test.db", PickleDbDumpPolicy::AutoDump);

    // set some values
    db.set("num", &100).unwrap();
    db.set("float", &1.1).unwrap();
    db.set("string", &String::from("my string")).unwrap();
    db.set("vec", &vec![1, 2, 3]).unwrap();
    db.lcreate("list1").unwrap().lextend(&[1, 2, 3]);

    // lock the DB file so no more writes will be possible
    let db_file = File::open("dump_error_test.db").unwrap();
    db_file.lock_exclusive().unwrap();

    // try set, confirm failure
    let try_set = db.set("num", &200);
    assert!(try_set.is_err());
    let try_set_err = try_set.err().unwrap();
    assert!(matches!(try_set_err.get_type(), ErrorType::Io));
    // verify the old value is still there
    assert_eq!(db.get::<i32>("num").unwrap(), 100);

    // try dump, confirm failure
    let try_dump = db.dump();
    assert!(try_dump.is_err());
    let try_dump_err = try_dump.err().unwrap();
    assert!(matches!(try_dump_err.get_type(), ErrorType::Io));

    // try rem, confirm failure
    let try_rem = db.rem("num");
    assert!(try_rem.is_err());
    let try_rem_err = try_rem.err().unwrap();
    assert!(matches!(try_rem_err.get_type(), ErrorType::Io));
    // verify "num" is still in the DB
    assert_eq!(db.get::<i32>("num").unwrap(), 100);

    // try lcreate, confirm failure
    let try_lcreate = db.lcreate("list2");
    assert!(try_lcreate.is_err());
    let try_lcreate_err = try_lcreate.err().unwrap();
    assert!(matches!(try_lcreate_err.get_type(), ErrorType::Io));

    // try ladd, confirm failure
    let try_ladd = db.ladd("list1", &100);
    assert!(try_ladd.is_none());
    // confirm list size is still the same
    assert_eq!(db.llen("list1"), 3);

    // try lextend, confirm failure
    let try_lextend = db.lextend("list1", &["aa", "bb"]);
    assert!(try_lextend.is_none());
    // confirm list size is still the same
    assert_eq!(db.llen("list1"), 3);

    // try lrem_list, confirm failure
    let try_lrem_list = db.lrem_list("list1");
    assert!(try_lrem_list.is_err());
    let try_lrem_list_err = try_lrem_list.err().unwrap();
    assert!(matches!(try_lrem_list_err.get_type(), ErrorType::Io));
    // verify "list1" is still in the DB
    assert!(db.exists("list1"));

    // try lpop, confirm failure
    let try_lpop = db.lpop::<i32>("list1", 0);
    assert!(try_lpop.is_none());
    // confirm list size is still the same
    assert_eq!(db.llen("list1"), 3);

    // try lrem_value, confirm failure
    let try_lrem_value = db.lrem_value("list1", &1);
    assert!(try_lrem_value.is_err());
    let try_lrem_value_err = try_lrem_value.err().unwrap();
    assert!(matches!(try_lrem_value_err.get_type(), ErrorType::Io));
    // verify "list1" is still in the DB
    assert_eq!(db.llen("list1"), 3);

    // unlock the file
    db_file.unlock().unwrap();
}
