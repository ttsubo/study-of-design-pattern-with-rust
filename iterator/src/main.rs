mod iterator;

use crate::iterator::aggregate::Aggregate;
use crate::iterator::book::{Book, BookShelf};

fn start_main() {
    let mut book_shelf = BookShelf::new();
    book_shelf.append(Book::new("Around the World in 80 days".to_string()));
    book_shelf.append(Book::new("Bible".to_string()));
    book_shelf.append(Book::new("Cinderella".to_string()));
    book_shelf.append(Book::new("Daddy-Long-Legs".to_string()));

    let mut it = book_shelf.iterator();
    while it.has_next() {
        let book = it.next();
        println!("{}", book.get_name());
    }
}

fn main() {
    start_main();
}
