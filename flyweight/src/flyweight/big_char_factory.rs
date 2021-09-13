use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

struct BigCharFactory {
    pool: HashMap<char, BigChar>,
}

impl BigCharFactory {
    fn new() -> BigCharFactory {
        BigCharFactory {
            pool: HashMap::new(),
        }
    }

    fn get_big_char(&mut self, charname: char) -> BigChar {
        let pool = self.pool.clone();
        match pool.get(&charname) {
            Some(bc) => bc.clone(),
            None => {
                let bc = BigChar::new(charname);
                self.pool.insert(charname, bc.clone());
                bc
            },
        }
    }
}

pub struct BigString {
    bigchars: Vec<BigChar>,
}

impl BigString {
    pub fn new(string: String) -> BigString {
        let mut bigchars = Vec::new();
        let mut factory = BigCharFactory::new();
        for ch in string.chars() {
            bigchars.push(factory.get_big_char(ch));
        }

        BigString { bigchars: bigchars }
    }

    pub fn print(&self) {
        for bc in &self.bigchars {
            bc.print();
        }
    }
}

#[derive(Clone)]
struct BigChar {
    fontdata: String,
}

impl BigChar {
    #[allow(unused_must_use)]
    fn new(charname: char) -> BigChar {
        let num: i32 = charname as i32 - 48;
        let exist = match num {
            0..=9 => true,
            _ => false,
        };
        let mut fontdata = String::new();
        if exist {
            let filename = format!("big{}.txt", charname);
            let mut f = File::open(filename).unwrap();
            f.read_to_string(&mut fontdata);
        } else {
            fontdata = format!("{}?", charname);
        };

        BigChar { fontdata: fontdata }
    }

    fn print(&self) {
        println!("{}", self.fontdata);
    }
}
