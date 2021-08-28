use crate::framework::prototype::Prototype;
use std::collections::HashMap;

pub trait ManagerTrait {
    fn register(&mut self, name: String, proto: Box<dyn Prototype>);
    fn create(&self, proto_name: String) -> Box<dyn Prototype>;
}

pub struct Manager {
    showcase: HashMap<String, Box<dyn Prototype>>,
}

impl Manager {
    pub fn new() -> Manager {
        Manager {
            showcase: HashMap::new(),
        }
    }
}

impl ManagerTrait for Manager {
    fn register(&mut self, name: String, proto: Box<dyn Prototype>) {
        self.showcase.insert(name, proto);
    }

    fn create(&self, proto_name: String) -> Box<dyn Prototype> {
        let prototype = self.showcase.get(&proto_name);
        match prototype {
            Some(p) => p.create_clone(),
            _ => panic!(),
        }
    }
}
