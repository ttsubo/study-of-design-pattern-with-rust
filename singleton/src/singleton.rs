use lazy_static::lazy_static;
use std::sync::RwLock;

lazy_static! {
    pub static ref GET_INSTANCE: RwLock<Singleton> = {
        RwLock::new(Singleton::new(1))
    };
}

pub struct Singleton {
    pub input: i32,
}

impl Singleton {
    pub fn new(val: i32) -> Singleton {
        println!("インスタンスを生成しました。(input={})", val);
        Singleton{input: val}
    }
    pub fn get_value(&self) -> i32 {
        self.input
    }
    pub fn set_value(&mut self, val: i32) {
        self.input = val
    }
}
