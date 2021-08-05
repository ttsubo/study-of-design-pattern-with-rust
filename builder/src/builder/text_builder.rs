use crate::builder::Builder;

pub struct TextBuilder {
    buffer: String,
}

impl TextBuilder {
    pub fn new() -> TextBuilder {
        TextBuilder {
            buffer: String::new(),
        }
    }
}

impl Builder for TextBuilder {
    fn make_title(&mut self, title: String) {
        self.buffer.push_str("======================\n");
        self.buffer.push_str(&format!("# {}\n", title));
        self.buffer.push_str("\n");
    }

    fn make_string(&mut self, str: String) {
        self.buffer.push_str(&format!("*** {} ***\n", str));
        self.buffer.push_str("\n");
    }

    fn make_items(&mut self, items: Vec<String>) {
        for i in &items {
            self.buffer.push_str(&format!("- {}\n", i));
        }
        self.buffer.push_str("\n");
    }

    fn close(&mut self) {
        self.buffer.push_str("======================\n");
    }

    fn get_result(&self) -> String {
        self.buffer.clone()
    }
}
