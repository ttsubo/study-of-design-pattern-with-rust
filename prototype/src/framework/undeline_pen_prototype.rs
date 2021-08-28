use crate::framework::prototype::Prototype;

#[derive(Clone)]
pub struct UnderlinePen {
    ul_char: char,
}

impl UnderlinePen {
    pub fn new(ul_char: char) -> UnderlinePen {
        UnderlinePen { ul_char: ul_char }
    }
}

impl Prototype for UnderlinePen {
    fn use_prototype(&self, s: String) {
        let length = s.chars().count();
        let mut ch = String::new();
        ch.push(self.ul_char);
        let line = ch.repeat(length + 2);

        println!("\"{}\"", s);
        println!("{}", line);
        println!("");
    }

    fn create_clone(&self) -> Box<dyn Prototype> {
        Box::new((*self).clone())
    }
}
