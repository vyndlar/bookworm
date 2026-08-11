use color_eyre::eyre::{Result, eyre};
use serde::Deserialize;

use super::book::Book;

#[derive(Debug, Deserialize)]
pub struct Series {
    pub title: String,
    pub books: Vec<Book>,
}
impl Series {
    pub fn new(title: String, books: Vec<Book>) -> Self {
        Self { title, books }
    }

    pub fn inferred_author(books: &[Book]) -> Result<String> {
        match books.first() {
            Some(book) => Ok(book.title.clone()),
            None => Err(eyre!("No books found!")),
        }
    }
}
