use super::{genre::Genre, series::Series};

#[derive(Debug)]
pub struct Book {
    total_pages: i16,
    author: String,
    title: String,
    times_read: i8,
    genre: Genre,
}

impl Book {
    pub fn new() -> Self {
        Book {
            total_pages: 0,
            author: String::from(""),
            title: String::from(""),
            times_read: 0,
            genre: Genre::DEFAULT,
        }
    }
}
