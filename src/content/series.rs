use super::book::Book;

#[derive(Debug)]
pub struct Series {
    title: String,
    books: Vec<Book>,
}

impl Series {
    pub fn new(title: String, books: Vec<Book>) -> Self {
        Self { title, books }
    }
}
