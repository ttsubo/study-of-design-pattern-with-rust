mod bridge;

use crate::bridge::display_count_func::DisplayCountFunc;
use crate::bridge::display_func::DisplayFunc;
use crate::bridge::display_random_func::DisplayRandomFunc;
use crate::bridge::display_string_impl::DisplayStringImpl;
use crate::bridge::display_texfile_impl::DisplayTextfileImpl;

fn start_main() {
    let mut d1 = DisplayFunc::new(Box::new(DisplayStringImpl::new("Hello, Japan".to_string())));
    let mut d2 =
        DisplayCountFunc::new(Box::new(DisplayStringImpl::new("Hello, Japan".to_string())));
    let mut d3 = DisplayCountFunc::new(Box::new(DisplayStringImpl::new(
        "Hello, Universe".to_string(),
    )));
    let mut d4 = DisplayRandomFunc::new(Box::new(DisplayStringImpl::new(
        "Hello, Universe".to_string(),
    )));
    let mut d5 = DisplayFunc::new(Box::new(DisplayTextfileImpl::new("test.txt".to_string())));
    d1.display();
    d2.display();
    d3.display();
    d3.multi_display(5);
    d4.random_display(5);
    d5.display();
}

fn main() {
    start_main();
}
