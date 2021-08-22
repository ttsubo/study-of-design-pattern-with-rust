use crate::framework::factory::{Factory, Product};

pub struct IDCardFactory {
    owners: Vec<String>,
}

impl IDCardFactory {
    pub fn new() -> IDCardFactory {
        IDCardFactory { owners: Vec::new() }
    }
}

impl Factory for IDCardFactory {
    fn create_product(&self, owner: String) -> Box<dyn Product> {
        Box::new(IDCardProduct::new(owner))
    }

    fn register_product(&mut self, product: &Box<dyn Product>) {
        self.owners.push(product.get_owner());
    }
}

struct IDCardProduct {
    owner: String,
}

impl IDCardProduct {
    fn new(owner: String) -> IDCardProduct {
        println!("I'll create {}'s card", owner);
        IDCardProduct { owner: owner }
    }
}

impl Product for IDCardProduct {
    fn use_product(&self) {
        println!("I'll use {}'s card", self.owner);
    }

    fn get_owner(&self) -> String {
        self.owner.clone()
    }
}
