use sqlite;

#[derive(Debug)]
pub enum DbError {
    SqError(sqlite::Error),
    //NotFound(String),
    InvalidCredentials
}

impl From<sqlite::Error> for DbError {
    fn from(sq_err: sqlite::Error) -> Self {
        DbError::SqError(sq_err)
    }
}