use std::fs;
use std::path::Path;

pub struct TestResources {
    file: String,
}

impl TestResources {
    pub fn new(file: &str) -> TestResources {
        TestResources {
            file: String::from(file),
        }
    }
}

impl Drop for TestResources {
    fn drop(&mut self) {
        let path = Path::new(&self.file);
        if path.exists() {
            let _ignore = fs::remove_file(path);
        }
    }
}

#[macro_export]
macro_rules! set_test_rsc {
    ($file_name:expr) => {
        let _test_rsc = common::TestResources::new($file_name);
    };
}

#[macro_export]
macro_rules! ser_method {
    ($ser_method_int:expr) => {
        SerializationMethod::from($ser_method_int)
    };
}

#[macro_export]
macro_rules! test_setup {
    ($function_name:expr, $ser_method_int:expr, $db_name:ident) => {
        let $db_name = format!(
            "{}_{}.db",
            $function_name,
            ser_method!($ser_method_int).to_string()
        );
        set_test_rsc!(&$db_name);
    };
}
