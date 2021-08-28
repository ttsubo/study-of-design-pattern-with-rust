pub trait Prototype {
    fn use_prototype(&self, s: String);
    fn create_clone(&self) -> Box<dyn Prototype>;
}
