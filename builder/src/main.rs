mod builder;

use crate::builder::director::Director;
use crate::builder::html_builder::HTMLBuilder;
use crate::builder::text_builder::TextBuilder;
use std::env;

fn start_main(opt: &str) {
    if opt == "plain" {
        let builder_obj = Box::new(TextBuilder::new());
        let mut director = Director::new(builder_obj);
        director.construct();
        let result = director.builder.get_result();
        println!("{}", result);
    } else if opt == "html" {
        let builder_obj = Box::new(HTMLBuilder::new("Greeting".to_string()));
        let mut director = Director::new(builder_obj);
        director.construct();
        let result = director.builder.get_result();
        println!("{}", result);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    start_main(args[1].as_str());
}
