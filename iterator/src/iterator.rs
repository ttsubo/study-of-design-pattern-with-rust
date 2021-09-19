pub mod aggregate;
pub mod book;

use crate::iterator::book::Book;

pub trait IteratorTrait {
    fn has_next(&self) -> bool;
    fn next(&mut self) -> Book;
}
