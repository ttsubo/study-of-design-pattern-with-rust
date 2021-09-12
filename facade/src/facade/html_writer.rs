use std::fs::File;
use std::io::prelude::*;

pub struct HtmlWriter {
    writer: File,
}

impl HtmlWriter {
    pub fn new(writer: File) -> HtmlWriter {
        HtmlWriter { writer: writer }
    }

    #[allow(unused_must_use)]
    pub fn title(&mut self, title: String) {
        write!(self.writer, "<html>\n");
        write!(self.writer, "<head>");
        write!(self.writer, "<title>{}</title>", title);
        write!(self.writer, "</head>\n");
        write!(self.writer, "<body>\n");
        write!(self.writer, "<h1>{}</h1>\n", title);
    }

    #[allow(unused_must_use)]
    pub fn paragraph(&mut self, msg: String) {
        write!(self.writer, "<p>{}</p>\n", msg);
    }

    #[allow(unused_must_use)]
    fn link(&mut self, href: String, caption: String) {
        write!(self.writer, "<a href=\"{}\">{}</a>", href, caption);
    }

    pub fn mailto(&mut self, mailaddr: String, username: String) {
        self.link(format!("mailto:{}", mailaddr), username);
    }

    #[allow(unused_must_use)]
    pub fn close(&mut self) {
        write!(self.writer, "</body>\n");
        write!(self.writer, "</html>\n");
    }
}
