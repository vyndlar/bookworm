use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum Genre {
    // fiction
    Fantasy,
    HistoricalFiction,
    Horror,
    Mystery,
    Romance,
    ScienceFiction,
    Action,
    Adventure,
    Dystopian,

    // nonfiction
    Autobiography,
    Biography,
    Cookbook,
    Religious,
    Education,
    Computers,
    Languages,
    Essay,
    Memoir,
    SelfHelp,
    Travel,
    TrueCrime,

    // other
    Poetry,
    Classic,
    Default, // default option (obviously)
}
