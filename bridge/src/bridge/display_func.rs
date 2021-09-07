use crate::bridge::display_impl::DisplayImpl;

pub struct DisplayFunc {
    pub imple: Box<dyn DisplayImpl>,
}

impl DisplayFunc {
    pub fn new(imple: Box<dyn DisplayImpl>) -> DisplayFunc {
        DisplayFunc { imple: imple }
    }

    pub fn open(&self) {
        self.imple.raw_open();
    }

    pub fn print_body(&mut self) {
        self.imple.raw_print();
    }

    pub fn close(&self) {
        self.imple.raw_close();
    }

    pub fn display(&mut self) {
        self.open();
        self.print_body();
        self.close();
    }
}
