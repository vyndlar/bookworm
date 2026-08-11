// master library struct

use serde::Deserialize;

use crate::content::series;

use super::{book::Book, series::Series};

#[derive(Debug, Deserialize)]
pub struct Library {
    pub series: Vec<Series>,

    pub standalone_books: Vec<Book>,
}

//impl Library {
//    fn get_books(&series: Series) -> &Vec<String> {}
//}
