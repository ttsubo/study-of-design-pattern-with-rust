use crate::decorator::display::Display;

struct Border {
    display: Box<dyn Display>,
}

pub struct SideBorder {
    border: Border,
    border_char: char,
}

impl SideBorder {
    pub fn new(display: Box<dyn Display>, border_char: char) -> SideBorder {
        SideBorder {
            border: Border { display: display },
            border_char: border_char,
        }
    }
}

impl Display for SideBorder {
    fn get_columns(&self) -> usize {
        1 + self.border.display.get_columns() + 1
    }

    fn get_rows(&self) -> usize {
        self.border.display.get_rows()
    }

    fn get_row_text(&self, row: usize) -> String {
        format!(
            "{}{}{}",
            self.border_char,
            self.border.display.get_row_text(row),
            self.border_char
        )
    }
}

pub struct FullBorder {
    border: Border,
}

impl FullBorder {
    pub fn new(display: Box<dyn Display>) -> FullBorder {
        FullBorder {
            border: Border { display: display },
        }
    }

    fn make_line(&self, ch: char, count: usize) -> String {
        let mut buf = "".to_string();
        for _ in 0..count {
            buf.push(ch);
        }
        buf
    }
}

impl Display for FullBorder {
    fn get_columns(&self) -> usize {
        1 + self.border.display.get_columns() + 1
    }

    fn get_rows(&self) -> usize {
        1 + self.border.display.get_rows() + 1
    }

    fn get_row_text(&self, row: usize) -> String {
        if row == 0 {
            format!(
                "+{}+",
                self.make_line('-', self.border.display.get_columns())
            )
        } else if row == self.border.display.get_rows() + 1 {
            format!(
                "+{}+",
                self.make_line('-', self.border.display.get_columns())
            )
        } else {
            format!("|{}|", self.border.display.get_row_text(row - 1))
        }
    }
}
