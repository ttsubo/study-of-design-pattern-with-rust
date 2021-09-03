mod adapter;

use crate::adapter::print_banner::PrintBanner;

fn start_main() {
    let p = PrintBanner::new("Hello".to_string());
    p.print_weak();
    p.print_strong();
}

fn main() {
    start_main();
}
