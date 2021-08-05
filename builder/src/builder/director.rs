use crate::builder::Builder;

pub struct Director {
    pub builder: Box<dyn Builder>,
}

impl Director {
    pub fn new(builder: Box<dyn Builder>) -> Director {
        Director { builder: builder }
    }

    pub fn construct(&mut self) {
        self.builder.make_title("Greeting".to_string());
        self.builder
            .make_string("From the morning to the afternoon".to_string());
        self.builder
            .make_items(vec!["Good morning".to_string(), "Hello".to_string()]);
        self.builder.make_string("In the evening".to_string());
        self.builder.make_items(vec![
            "Good evening".to_string(),
            "Good night".to_string(),
            "Good bye".to_string(),
        ]);
        self.builder.close();
    }
}
