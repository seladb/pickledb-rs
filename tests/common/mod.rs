use std::path::Path;
use std::fs;

pub struct TestResources {
    file: String,
}

impl TestResources {
    pub fn new(file: &str) -> TestResources {
        TestResources { file: String::from(file) }
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
macro_rules! test_setup {
    ($function_name:expr, $ser_method:expr, $db_name:ident) => {
        let $db_name = format!("{}_{}.db", $function_name, $ser_method.to_string());
        set_test_rsc!(&$db_name);
    }
}