// master library struct

use serde::Deserialize;

use super::{book::Book, series::Series};

#[derive(Debug, Deserialize)]
pub struct Library {
    series: Vec<Series>,

    standalone_books: Vec<Book>,
}
