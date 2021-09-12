use crate::facade::database::Database;
use crate::facade::html_writer::HtmlWriter;
use std::fs::File;

pub struct PageMaker {}

impl PageMaker {
    pub fn make_welcome_page(mailaddr: String, filename: String) {
        let prop = Database::get_properties("maildata".to_string());
        let username = match prop.get(&mailaddr) {
            Some(n) => n,
            None => panic!("{} is not found", mailaddr),
        };

        let file = File::create(filename.clone()).expect("Unable to create file");
        let mut writer = HtmlWriter::new(file);
        writer.title(format!("Welcome to {}'s page!", username));
        writer.paragraph("We'll wait for your sending".to_string());
        writer.mailto(mailaddr.clone(), username.to_string());
        writer.close();
        println!("{} is created for {} ({})", filename, mailaddr, username);
    }
}
