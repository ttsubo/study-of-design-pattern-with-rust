use crate::bridge::display_impl::DisplayImpl;

pub struct DisplayStringImpl {
    str: String,
    width: usize,
}

impl DisplayStringImpl {
    pub fn new(str: String) -> DisplayStringImpl {
        DisplayStringImpl {
            str: str.clone(),
            width: str.chars().count(),
        }
    }

    fn print_line(&self) {
        print!("+");
        for _ in 0..self.width {
            print!("-");
        }
        println!("+");
    }
}

impl DisplayImpl for DisplayStringImpl {
    fn raw_open(&self) {
        self.print_line();
    }

    fn raw_print(&mut self) {
        println!("|{}|", self.str);
    }

    fn raw_close(&self) {
        self.print_line();
        println!("");
    }
}
