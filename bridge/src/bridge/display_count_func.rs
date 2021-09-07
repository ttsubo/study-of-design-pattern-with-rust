use crate::bridge::display_func::DisplayFunc;
use crate::bridge::display_impl::DisplayImpl;

pub struct DisplayCountFunc {
    display: DisplayFunc,
}

impl DisplayCountFunc {
    pub fn new(imple: Box<dyn DisplayImpl>) -> DisplayCountFunc {
        DisplayCountFunc {
            display: DisplayFunc { imple: imple },
        }
    }

    pub fn multi_display(&mut self, times: u32) {
        self.display.open();
        for _ in 0..times {
            self.display.print_body();
        }
        self.display.close();
    }

    pub fn display(&mut self) {
        self.display.open();
        self.display.print_body();
        self.display.close();
    }
}
