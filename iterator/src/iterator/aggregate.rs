use crate::iterator::IteratorTrait;

pub trait Aggregate {
    fn iterator(&self) -> Box<dyn IteratorTrait>;
}
