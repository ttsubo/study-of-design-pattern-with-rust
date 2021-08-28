use crate::framework::prototype::Prototype;

#[derive(Clone)]
pub struct MessageBox {
    deco_char: char,
}

impl MessageBox {
    pub fn new(deco_char: char) -> MessageBox {
        MessageBox {
            deco_char: deco_char,
        }
    }
}

impl Prototype for MessageBox {
    fn use_prototype(&self, s: String) {
        let length = s.chars().count();
        let mut ch = String::new();
        ch.push(self.deco_char);
        let line = ch.repeat(length + 4);

        println!("{}", line);
        println!("{} {} {}", self.deco_char, s, self.deco_char);
        println!("{}", line);
        println!("");
    }

    fn create_clone(&self) -> Box<dyn Prototype> {
        Box::new((*self).clone())
    }
}
