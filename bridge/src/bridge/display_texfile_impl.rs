use crate::bridge::display_impl::DisplayImpl;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub struct DisplayTextfileImpl {
    file_name: String,
    f: File,
}

impl DisplayTextfileImpl {
    pub fn new(file_name: String) -> DisplayTextfileImpl {
        let path = Path::new(&file_name);
        let file = match File::open(&path) {
            Err(err) => panic!("couldn't open {}: {}", file_name, err.to_string()),
            Ok(file) => file,
        };
        DisplayTextfileImpl {
            file_name: file_name,
            f: file,
        }
    }
}

impl DisplayImpl for DisplayTextfileImpl {
    fn raw_open(&self) {}

    fn raw_print(&mut self) {
        let mut buf = String::new();
        match self.f.read_to_string(&mut buf) {
            Err(err) => panic!("couldn't read {}: {}", self.file_name, err.to_string()),
            Ok(_) => println!("{}", buf),
        }
    }

    fn raw_close(&self) {
        self.f.sync_all().unwrap();
    }
}
