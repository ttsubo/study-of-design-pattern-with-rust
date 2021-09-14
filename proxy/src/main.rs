mod proxy;

use crate::proxy::printer_proxy::{Printable, PrinterProxy};

fn start_main(mut p: Box<dyn Printable>) {
    println!("Printer代理人の名前は現在({})です", p.get_printer_name());
    p.my_print("Nice to meet you".to_string());
    p.set_printer_name("Bob".to_string());
    println!("Printer代理人の名前は現在({})です", p.get_printer_name());
    p.my_print("Hello, world".to_string());
}

fn main() {
    start_main(Box::new(PrinterProxy::new("Alice".to_string())));
}
