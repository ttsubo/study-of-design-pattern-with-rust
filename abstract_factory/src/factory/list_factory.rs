use crate::factory::{Factory, Item, ItemTrait, Link, Page, PageTrait, Tray};
use std::fs::File;
use std::io::prelude::*;

pub struct ListFactory {}

impl Factory for ListFactory {
    fn create_link(&self, caption: String, url: String) -> Box<dyn ItemTrait> {
        Box::new(ListLink::new(caption, url))
    }

    fn create_tray(&self, caption: String) -> Box<dyn ItemTrait> {
        Box::new(ListTray::new(caption))
    }

    fn create_page(&self, title: String, author: String) -> Box<dyn PageTrait> {
        Box::new(ListPage::new(title, author))
    }
}

pub struct ListLink {
    link: Link,
}

impl ListLink {
    pub fn new(caption: String, url: String) -> ListLink {
        ListLink {
            link: Link {
                item: Item { caption: caption },
                url: url,
            },
        }
    }
}

impl ItemTrait for ListLink {
    fn make_html(&self) -> String {
        format!(
            "<li><a href=\"{}\">{}</a></li>\n",
            self.link.url, self.link.item.caption
        )
    }

    // dummy method
    #[warn(unused_variables)]
    fn add(&mut self, _item: Box<dyn ItemTrait>) {}
}

pub struct ListTray {
    tray: Tray,
}

impl ListTray {
    pub fn new(caption: String) -> ListTray {
        ListTray {
            tray: Tray {
                item: Item { caption: caption },
                tray: Vec::new(),
            },
        }
    }
}

impl ItemTrait for ListTray {
    fn make_html(&self) -> String {
        let mut buffer = "".to_string();

        buffer.push_str("<li>\n");
        buffer.push_str(&format!("{}\n", &self.tray.item.caption));
        buffer.push_str("<ul>\n");
        for item in &self.tray.tray {
            buffer.push_str(&item.make_html());
        }
        buffer.push_str("</ul>\n");
        buffer.push_str("</li>\n");

        buffer
    }

    fn add(&mut self, item: Box<dyn ItemTrait>) {
        self.tray.tray.push(item);
    }
}

pub struct ListPage {
    page: Page,
}

impl ListPage {
    pub fn new(title: String, author: String) -> ListPage {
        ListPage {
            page: Page {
                title: title,
                author: author,
                content: Vec::new(),
            },
        }
    }
}

impl ItemTrait for ListPage {
    fn make_html(&self) -> String {
        let mut buffer = "".to_string();

        buffer.push_str(&format!(
            "<html><head><title>{}</title></head>\n",
            self.page.title
        ));
        buffer.push_str("<body>\n");
        buffer.push_str(&format!("<h1>{}</h1>\n", self.page.title));
        buffer.push_str("<ul>\n");
        for item in &self.page.content {
            buffer.push_str(&item.make_html());
        }
        buffer.push_str("</ul>\n");
        buffer.push_str(&format!("<hr><address>{}</address>", self.page.author));
        buffer.push_str("</body></html>\n");

        buffer
    }

    fn add(&mut self, tray: Box<dyn ItemTrait>) {
        self.page.content.push(tray);
    }
}

impl PageTrait for ListPage {
    #[allow(unused_must_use)]
    fn output(&self) {
        let file_name = format!("{}.html", self.page.title);
        let mut writer = File::create(file_name.clone()).expect("Unable to create file");

        writeln!(writer, "{}", self.make_html());
        println!("{} was created.", file_name);
    }
}
