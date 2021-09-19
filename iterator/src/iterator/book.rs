use crate::iterator::aggregate::Aggregate;
use crate::iterator::IteratorTrait;

#[derive(Clone)]
pub struct Book {
    name: String,
}

impl Book {
    pub fn new(name: String) -> Book {
        Book { name: name }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

struct BookShelfIterator {
    book_shelf: BookShelf,
    index: u32,
}

impl BookShelfIterator {
    fn new(book_shelf: BookShelf) -> BookShelfIterator {
        BookShelfIterator {
            book_shelf: book_shelf,
            index: 0,
        }
    }
}

impl IteratorTrait for BookShelfIterator {
    fn has_next(&self) -> bool {
        if self.index < self.book_shelf.get_length() {
            true
        } else {
            false
        }
    }

    fn next(&mut self) -> Book {
        let book = self.book_shelf.get_book_at(self.index);
        self.index += 1;
        book
    }
}

#[derive(Clone)]
pub struct BookShelf {
    last: u32,
    books: Vec<Book>,
}

impl BookShelf {
    pub fn new() -> BookShelf {
        BookShelf {
            books: Vec::new(),
            last: 0,
        }
    }

    fn get_book_at(&self, index: u32) -> Book {
        self.books[index as usize].clone()
    }

    pub fn append(&mut self, book: Book) {
        self.books.push(book);
        self.last += 1;
    }

    fn get_length(&self) -> u32 {
        self.last
    }
}

impl Aggregate for BookShelf {
    fn iterator(&self) -> Box<dyn IteratorTrait> {
        Box::new(BookShelfIterator::new((*self).clone()))
    }
}
