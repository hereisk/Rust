use crate::db_data::db_error::DbError;

use sqlite::Connection;

const DB_PATH: &str = "./data/db.sql";

fn db_init() -> Result<Connection, DbError> {
    let connection = sqlite::open(DB_PATH)?;
    connection.execute(
        "create table if not exists users(
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            firstname VARCHAR(25) NOT NULL,
            lastname VARCHAR(25) NOT NULL,
            age INTEGER NOT NULL,
            password VARCHAR(25) NOT NULL,
            email VARCHAR(25) NOT NULL UNIQUE
        );"
    )?;
    Ok(connection)
}

pub fn db_connect() -> Result<Connection, DbError> {
    db_init()
}