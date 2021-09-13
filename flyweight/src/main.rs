mod flyweight;

use crate::flyweight::big_char_factory::BigString;
use std::env;

fn start_main(opt: String) {
    let bs = BigString::new(opt);
    bs.print();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    start_main(args[1].clone());
}
