use super::book::Book;

#[derive(Debug)]
pub struct Series {
    title: String,
    total_books: i8,
    books: Vec<Book>,
}

impl Series {
    pub fn new(title: String, total_books: i8, books: Vec<Book>) -> Self {
        Self {
            title,
            total_books,
            books,
        }
    }
}
