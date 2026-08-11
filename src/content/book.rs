use serde::Deserialize;

use super::genre::Genre;

#[derive(Debug, Deserialize)]
pub struct Book {
    pub total_pages: i16,
    pub author: String,
    pub title: String,
    pub times_read: i8,
    pub genre: Genre,
    pub is_owned: bool, // do you own the book or is it borrowed?
}

impl Book {
    pub fn new() -> Self {
        Book {
            total_pages: 0,
            author: String::from(""),
            title: String::from(""),
            times_read: 0,
            genre: Genre::Default,
            is_owned: false,
        }
    }
}
