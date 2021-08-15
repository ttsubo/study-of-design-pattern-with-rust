use crate::factory::{Factory, Item, ItemTrait, Link, Page, PageTrait, Tray, TrayTrait};
use std::fs::File;
use std::io::prelude::*;

pub struct TableFactory {}

impl Factory for TableFactory {
    fn create_link(&self, caption: String, url: String) -> Box<dyn TrayTrait> {
        Box::new(TableLink::new(caption, url))
    }

    fn create_tray(&self, caption: String) -> Box<dyn TrayTrait> {
        Box::new(TableTray::new(caption))
    }

    fn create_page(&self, title: String, author: String) -> Box<dyn PageTrait> {
        Box::new(TablePage::new(title, author))
    }
}

struct TableLink {
    link: Link,
}

impl TableLink {
    fn new(caption: String, url: String) -> TableLink {
        TableLink {
            link: Link {
                item: Item { caption: caption },
                url: url,
            },
        }
    }
}

impl ItemTrait for TableLink {
    fn make_html(&self) -> String {
        format!(
            "<td><a href=\"{}\">{}</a></td>\n",
            self.link.url, self.link.item.caption
        )
    }
}

impl TrayTrait for TableLink {
    // dummy method
    fn add(&mut self, _item: Box<dyn TrayTrait>) {}
}

struct TableTray {
    tray: Tray,
}

impl TableTray {
    fn new(caption: String) -> TableTray {
        TableTray {
            tray: Tray {
                item: Item { caption: caption },
                tray: Vec::new(),
            },
        }
    }
}

impl ItemTrait for TableTray {
    fn make_html(&self) -> String {
        let mut buffer = "".to_string();

        buffer.push_str("<td>");
        buffer.push_str("<table width=\"100%\" border=\"1\"<tr>");
        buffer.push_str(&format!(
            "<td bgcolor=\"#cccccc\" align=\"center\"
        colspan=\"{}\"><b>{}</b></td>",
            self.tray.tray.len(),
            self.tray.item.caption
        ));
        buffer.push_str("</tr>\n");
        buffer.push_str("<tr>\n");
        for item in &self.tray.tray {
            buffer.push_str(&item.make_html());
        }
        buffer.push_str("</tr></table>");
        buffer.push_str("</td>");

        buffer
    }
}

impl TrayTrait for TableTray {
    fn add(&mut self, item: Box<dyn TrayTrait>) {
        self.tray.tray.push(item);
    }
}

struct TablePage {
    page: Page,
}

impl TablePage {
    fn new(title: String, author: String) -> TablePage {
        TablePage {
            page: Page {
                title: title,
                author: author,
                content: Vec::new(),
            },
        }
    }
}

impl ItemTrait for TablePage {
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
}

impl PageTrait for TablePage {
    fn add(&mut self, tray: Box<dyn TrayTrait>) {
        self.page.content.push(tray);
    }

    #[allow(unused_must_use)]
    fn output(&self) {
        let file_name = format!("{}.html", self.page.title);
        let mut writer = File::create(file_name.clone()).expect("Unable to create file");

        writeln!(writer, "{}", self.make_html());
        println!("{} was created.", file_name);
    }
}
