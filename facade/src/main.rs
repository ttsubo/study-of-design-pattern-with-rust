mod facade;

use crate::facade::page_maker::PageMaker;

fn start_main() {
    PageMaker::make_welcome_page("hyuki@hyuki.com".to_string(), "welcome1.html".to_string());
    PageMaker::make_welcome_page("mamoru@hyuki.com".to_string(), "welcome2.html".to_string());
}

fn main() {
    start_main();
}
