//use std::fmt;
use crate::composite::entry::Entry;

pub struct Directory {
    name: String,
    directory: Vec<Box<dyn Entry>>,
}

impl Directory {
    pub fn new(name: String) -> Directory {
        Directory {
            name: name,
            directory : Vec::new(), 
        }
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn add(&mut self, entry: Box<dyn Entry>) {
        self.directory.push(entry);
    }

    fn print(&self, prefix: String){
        println!("{}/{} ({})", prefix, self.get_name(), self.get_size());
    }
}

impl Entry for Directory {
    fn get_size(&self) -> u32 {
        let mut size = 0;
        for entry in &self.directory {
            size += entry.get_size()
        }
        size
    }

    fn print_list(&self, prefix: String) {
        self.print(prefix.clone());
        for e in &self.directory {
            e.print_list(format!("{}/{}", prefix, self.name));
        }
    }
}
