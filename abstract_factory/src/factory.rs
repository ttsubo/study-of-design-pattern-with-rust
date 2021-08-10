pub mod list_factory;
pub mod table_factory;

pub trait Factory {
    fn create_link(&self, caption: String, url: String) -> Box<dyn TrayTrait>;
    fn create_tray(&self, caption: String) -> Box<dyn TrayTrait>;
    fn create_page(&self, title: String, author: String) -> Box<dyn PageTrait>;
}

pub trait ItemTrait {
    fn make_html(&self) -> String;
}

pub struct Item {
    caption: String,
}

pub struct Link {
    item: Item,
    url: String,
}

pub trait TrayTrait: ItemTrait {
    fn add(&mut self, item: Box<dyn TrayTrait>);
}

pub struct Tray {
    item: Item,
    tray: Vec<Box<dyn TrayTrait>>,
}

pub trait PageTrait: ItemTrait {
    fn add(&mut self, item: Box<dyn TrayTrait>);
    fn output(&self);
}

pub struct Page {
    title: String,
    author: String,
    content: Vec<Box<dyn TrayTrait>>,
}
