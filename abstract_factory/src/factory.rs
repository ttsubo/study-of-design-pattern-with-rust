pub mod list_factory;
pub mod table_factory;

pub trait Factory {
    fn create_link(&self, caption: String, url: String) -> Box<dyn ItemTrait>;
    fn create_tray(&self, caption: String) -> Box<dyn ItemTrait>;
    fn create_page(&self, title: String, author: String) -> Box<dyn PageTrait>;
}

pub struct Item {
    caption: String,
}

pub struct Link {
    item: Item,
    url: String,
}

pub struct Tray {
    item: Item,
    tray: Vec<Box<dyn ItemTrait>>,
}

pub struct Page {
    title: String,
    author: String,
    content: Vec<Box<dyn ItemTrait>>,
}

pub trait ItemTrait {
    fn make_html(&self) -> String;
    fn add(&mut self, item: Box<dyn ItemTrait>);
}

pub trait PageTrait: ItemTrait {
    fn output(&self);
}
