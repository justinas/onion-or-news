#![deny(clippy::all)]

pub mod models;
pub mod schema;

use std::fmt;

#[macro_use]
extern crate diesel;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PoolError};
#[macro_use]
extern crate diesel_migrations;
use rand::RngCore;

embed_migrations!();

#[derive(Debug)]
pub enum Error {
    Diesel(diesel::result::Error),
    Migration(diesel_migrations::RunMigrationsError),
    Pool(PoolError),
    NotFound,
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

impl From<diesel_migrations::RunMigrationsError> for Error {
    fn from(e: diesel_migrations::RunMigrationsError) -> Self {
        Error::Migration(e)
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

    pub fn migrate(&self) -> Result<(), Error> {
        let conn = self.pool.get()?;
        Ok(embedded_migrations::run(&conn)?)
    }

    pub fn get_question(&self, id: uuid::Uuid) -> Result<models::Question, Error> {
        let conn = self.pool.get()?;
        let results = schema::questions::table
            .filter(schema::questions::id.eq(id))
            .limit(1)
            .load::<models::Question>(&conn)?;
        match results.into_iter().next() {
            Some(q) => Ok(q),
            None => Err(Error::NotFound),
        }
    }

    pub fn get_random_question(&self) -> Result<models::Question, Error> {
        no_arg_sql_function!(RANDOM, diesel::sql_types::Float, "RANDOM");

        let conn = self.pool.get()?;
        let choice_id = ((rand::thread_rng().next_u32() as i32) & 1) + 1;
        let results = schema::questions::table
            .order(RANDOM)
            .filter(schema::questions::choice_id.eq(choice_id))
            .limit(1)
            .load::<models::Question>(&conn)?;
        match results.into_iter().next() {
            Some(q) => Ok(q),
            None => Err(Error::NotFound),
        }
    }

    pub fn insert_question(&self, question: &models::NewQuestion) -> Result<usize, Error> {
        let conn = self.pool.get()?;
        Ok(diesel::insert_into(schema::questions::table)
            .values(question)
            .on_conflict_do_nothing()
            .execute(&conn)?)
    }

    pub fn insert_answer(
        &self,
        ip: std::net::IpAddr,
        question_id: uuid::Uuid,
        choice_id: i32,
    ) -> Result<usize, Error> {
        let conn = self.pool.get()?;
        let ipnet = ip.into();
        let answer = models::NewAnswer {
            ip: &ipnet,
            question_id: &question_id,
            choice_id,
        };
        Ok(diesel::insert_into(schema::answers::table)
            .values(&answer)
            .on_conflict_do_nothing()
            .execute(&conn)?)
    }
}
