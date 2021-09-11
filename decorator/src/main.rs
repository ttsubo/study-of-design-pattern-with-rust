mod decorator;

use crate::decorator::border::{FullBorder, SideBorder};
use crate::decorator::display::{Display, StringDisplay};

fn start_main() {
    let b1 = StringDisplay::new("Hello, world.".to_string());
    b1.show();
    let b2 = SideBorder::new(Box::new(b1), '#');
    b2.show();
    let b3 = FullBorder::new(Box::new(b2));
    b3.show();
    println!("");
    let b4 = SideBorder::new(
        Box::new(FullBorder::new(Box::new(FullBorder::new(Box::new(
            SideBorder::new(
                Box::new(FullBorder::new(Box::new(StringDisplay::new(
                    "HELLO".to_string(),
                )))),
                '*',
            ),
        ))))),
        '/',
    );
    b4.show();
}

fn main() {
    start_main();
}
