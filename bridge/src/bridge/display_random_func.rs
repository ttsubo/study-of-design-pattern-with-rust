use crate::bridge::display_func::DisplayFunc;
use crate::bridge::display_impl::DisplayImpl;
use rand::Rng;

pub struct DisplayRandomFunc {
    display: DisplayFunc,
}

impl DisplayRandomFunc {
    pub fn new(imple: Box<dyn DisplayImpl>) -> DisplayRandomFunc {
        DisplayRandomFunc {
            display: DisplayFunc { imple: imple },
        }
    }

    pub fn random_display(&mut self, times: u32) {
        let mut rng = rand::thread_rng();
        let random_times = rng.gen_range(0..times);
        self.display.open();
        for _ in 0..random_times {
            self.display.print_body();
        }
        self.display.close();
    }
}
