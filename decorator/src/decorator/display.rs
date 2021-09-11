pub trait Display {
    fn get_columns(&self) -> usize;
    fn get_rows(&self) -> usize;
    fn get_row_text(&self, row: usize) -> String;
    fn show(&self) {
        for i in 0..self.get_rows() {
            println!("{}", self.get_row_text(i));
        }
    }
}

pub struct StringDisplay {
    string: String,
}

impl StringDisplay {
    pub fn new(string: String) -> StringDisplay {
        StringDisplay { string: string }
    }
}

impl Display for StringDisplay {
    fn get_columns(&self) -> usize {
        self.string.len()
    }

    fn get_rows(&self) -> usize {
        1
    }

    fn get_row_text(&self, row: usize) -> String {
        if row == 0 {
            self.string.clone()
        } else {
            "".to_string()
        }
    }
}
