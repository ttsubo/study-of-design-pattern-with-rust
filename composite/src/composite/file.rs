use crate::composite::entry::Entry;

pub struct File {
    name: String,
    size: u32,
}

impl File {
    pub fn new(name: String, size: u32) -> File {
        File {
            name: name,
            size: size,
        }
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn print(&self, prefix: String){
        println!("{}/{} ({})", prefix, self.get_name(), self.get_size());
    }
}

impl Entry for File {
    fn get_size(&self) -> u32 {
        self.size
    }

    fn print_list(&self, prefix: String) {
        self.print(prefix.clone());
    }
}
