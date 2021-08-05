use crate::builder::Builder;
use std::fs::File;
use std::io::prelude::*;
use std::process;

pub struct HTMLBuilder {
    file_name: String,
    f: File,
    make_title_called: bool,
}

impl HTMLBuilder {
    pub fn new(title: String) -> HTMLBuilder {
        let file_name = format!("{}.html", title);
        let f = File::create(file_name.clone()).expect("Unable to create file");

        HTMLBuilder {
            file_name: file_name,
            f: f,
            make_title_called: false,
        }
    }
}

#[allow(unused_must_use)]
impl Builder for HTMLBuilder {
    fn make_title(&mut self, title: String) {
        self.file_name = format!("{}.html", title);
        writeln!(self.f, "<html><head><title>{}</title></head><body>", title);
        writeln!(self.f, "<h1>{}</h1>", title);
        self.make_title_called = true;
    }

    fn make_string(&mut self, string: String) {
        if !self.make_title_called {
            process::exit(1);
        }
        writeln!(self.f, "<p>{}</p>", string);
    }

    fn make_items(&mut self, items: Vec<String>) {
        if !self.make_title_called {
            process::exit(1);
        }
        writeln!(self.f, "<ul>");
        for i in &items {
            writeln!(self.f, "<li>{}</li>", i);
        }
        writeln!(self.f, "</ul>");
    }

    fn close(&mut self) {
        if !self.make_title_called {
            process::exit(1);
        }
        writeln!(self.f, "</body></html>");
        self.f.flush();
    }

    fn get_result(&self) -> String {
        self.file_name.clone()
    }
}
