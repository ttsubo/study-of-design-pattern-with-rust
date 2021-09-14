use std::io::{self, Write};
use std::{thread, time};

pub trait Printable {
    fn set_printer_name(&mut self, name: String);
    fn get_printer_name(&self) -> String;
    fn my_print(&mut self, string: String);
}

struct Printer {
    name: String,
}

impl Printer {
    fn new(name: String) -> Printer {
        Printer::heaby_job(format!("Printerのインスタンス({})を生成中", name));
        Printer { name: name }
    }

    fn heaby_job(msg: String) {
        print!("{}", msg);
        io::stdout().flush().unwrap();
        for _ in 0..10 {
            thread::sleep(time::Duration::from_millis(1000));
            print!(".");
            io::stdout().flush().unwrap();
        }
        println!("完了");
    }
}

impl Printable for Printer {
    fn set_printer_name(&mut self, name: String) {
        self.name = name;
    }

    fn get_printer_name(&self) -> String {
        self.name.clone()
    }

    fn my_print(&mut self, string: String) {
        println!("=== Printer使用者({}) ===", self.name);
        println!("{}", string);
        println!("")
    }
}

pub struct PrinterProxy {
    name: String,
    real: Option<Printer>,
}

impl PrinterProxy {
    pub fn new(name: String) -> PrinterProxy {
        PrinterProxy {
            name: name,
            real: None,
        }
    }
}

impl Printable for PrinterProxy {
    fn set_printer_name(&mut self, name: String) {
        self.name = name;
    }

    fn get_printer_name(&self) -> String {
        self.name.clone()
    }

    fn my_print(&mut self, string: String) {
        if self.real.is_none() {
            self.real = Some(Printer::new(self.name.clone()));
        } else {
            match self.real.as_mut() {
                Some(real) => real.set_printer_name(self.name.clone()),
                None => {}
            }
        }
        match self.real.as_mut() {
            Some(real) => real.my_print(string),
            None => {}
        }
    }
}
