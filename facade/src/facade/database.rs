use csv;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

pub struct Database {}

#[derive(Deserialize)]
struct Record {
    mail_addr: String,
    user_name: String,
}

impl Database {
    pub fn get_properties(db_name: String) -> HashMap<String, String> {
        let mut maildata = HashMap::new();
        let file_name = format!("{}.txt", db_name);
        let path = Path::new(&file_name);
        let file = match File::open(&path) {
            Err(err) => panic!("couldn't open {}: {}", file_name, err.to_string()),
            Ok(file) => file,
        };
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(file);

        for result in rdr.deserialize() {
            let record: Record = result.unwrap();
            maildata.insert(record.mail_addr, record.user_name);
        }
        maildata
    }
}
