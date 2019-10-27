pub mod models;
pub mod schema;

use std::fmt;

#[macro_use]
extern crate diesel;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PoolError};

#[derive(Debug)]
pub enum Error {
    Diesel(diesel::result::Error),
    Pool(diesel::r2d2::PoolError),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<diesel::result::Error> for Error {
    fn from(e: diesel::result::Error) -> Self {
        Error::Diesel(e)
    }
}

impl From<PoolError> for Error {
    fn from(e: PoolError) -> Self {
        Error::Pool(e)
    }
}

pub struct Database {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl Database {
    pub fn connect(conn: &str) -> Result<Self, Error> {
        let pool = Pool::new(ConnectionManager::<PgConnection>::new(conn))?;
        Ok(Database { pool })
    }

    pub fn insert_question(&self, question: &models::NewQuestion) -> Result<usize, Error> {
        let conn = self.pool.get().unwrap();
        Ok(diesel::insert_into(schema::questions::table)
            .values(question)
            .on_conflict_do_nothing()
            .execute(&conn)?)
    }
}
