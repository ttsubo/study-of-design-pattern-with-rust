use crate::adapter::banner::{Banner, BannerTrait};

pub struct PrintBanner {
    banner: Box<dyn BannerTrait>,
}

impl PrintBanner {
    pub fn new(str: String) -> PrintBanner {
        PrintBanner {
            banner: Box::new(Banner::new(str)),
        }
    }

    pub fn print_weak(&self) {
        self.banner.show_with_paren();
    }

    pub fn print_strong(&self) {
        self.banner.show_with_aster();
    }
}
