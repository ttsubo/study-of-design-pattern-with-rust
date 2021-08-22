pub trait Factory {
    fn create_product(&self, owner: String) -> Box<dyn Product>;
    fn register_product(&mut self, product: &Box<dyn Product>);
    fn create(&mut self, owner: String) -> Box<dyn Product> {
        let product = self.create_product(owner);
        self.register_product(&product);
        product
    }
}

pub trait Product {
    fn use_product(&self);
    fn get_owner(&self) -> String;
}
