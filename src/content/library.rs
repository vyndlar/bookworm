// master library struct

use super::{book::Book, series::Series};

#[derive(Debug)]
pub struct Library {
    series: Vec<Series>,

    standalone_books: Vec<Book>,
}
