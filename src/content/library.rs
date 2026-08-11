// master library struct

use serde::Deserialize;

use super::{book::Book, series::Series};

#[derive(Debug, Deserialize)]
pub struct Library {
    pub series: Vec<Series>,

    pub standalone_books: Vec<Book>,
}
