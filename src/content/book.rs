use serde::Deserialize;

use super::genre::Genre;

#[derive(Debug, Deserialize)]
pub struct Book {
    total_pages: i16,
    author: String,
    title: String,
    times_read: i8,
    genre: Genre,
    is_owned: bool, // do you own the book or is it borrowed?
}

impl Book {
    pub fn new() -> Self {
        Book {
            total_pages: 0,
            author: String::from(""),
            title: String::from(""),
            times_read: 0,
            genre: Genre::DEFAULT,
            is_owned: false,
        }
    }
}
