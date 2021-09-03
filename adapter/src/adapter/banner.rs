pub trait BannerTrait {
    fn show_with_paren(&self);
    fn show_with_aster(&self);
}

pub struct Banner {
    str: String,
}

impl Banner {
    pub fn new(str: String) -> Banner {
        Banner { str: str }
    }
}

impl BannerTrait for Banner {
    fn show_with_paren(&self) {
        println!("({})", self.str);
    }

    fn show_with_aster(&self) {
        println!("*{}*", self.str);
    }
}
