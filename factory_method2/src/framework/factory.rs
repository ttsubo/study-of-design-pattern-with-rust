/*
pub trait FactoryTrait {
    fn create_product(&self, owner: String) -> Box<dyn Product>;
    fn register_product(&mut self, product: Box<dyn Product>);
    fn create(&mut self, owner: String) -> Box<dyn Product>;
}

pub struct Factory {
    pub factory: Box<dyn FactoryTrait>,
}
*/

pub trait Product {
    fn use_product(&self);
    fn get_owner(&self) -> String;
}
