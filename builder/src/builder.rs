pub mod director;
pub mod html_builder;
pub mod text_builder;

pub trait Builder {
    fn make_title(&mut self, title: String);
    fn make_string(&mut self, string: String);
    fn make_items(&mut self, items: Vec<String>);
    fn close(&mut self);
    fn get_result(&self) -> String;
}
