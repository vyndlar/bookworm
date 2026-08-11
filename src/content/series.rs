use serde::Deserialize;

use super::book::Book;

#[derive(Debug, Deserialize)]
pub struct Series {
    title: String,
    books: Vec<Book>,
}

impl Series {
    pub fn new(title: String, books: Vec<Book>) -> Self {
        Self { title, books }
    }
}
