#[derive(Clone)]
pub struct Trouble {
    number: u32,
}

impl Trouble {
    pub fn new(number: u32) -> Trouble {
        Trouble { number: number }
    }

    pub fn get_number(&self) -> u32 {
        self.number
    }

    pub fn print(&self) -> String {
        format!("[Trouble {}]", self.number)
    }
}
