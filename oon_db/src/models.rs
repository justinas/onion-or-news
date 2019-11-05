use chrono::{naive::NaiveDateTime, DateTime, Utc};
use diesel::{Insertable, Queryable, QueryableByName};
use uuid::Uuid;

use super::schema::*;

#[derive(Debug, Insertable)]
#[table_name = "questions"]
pub struct NewQuestion<'a> {
    pub id: &'a Uuid,
    pub foreign_id: &'a str,
    pub title: &'a str,
    pub choice_id: i32,
    pub meta_url: &'a str,
    pub url: &'a str,
    pub thumbnail: Option<&'a str>,
}

#[derive(Queryable, QueryableByName)]
#[table_name = "questions"]
pub struct Question {
    pub id: Uuid,
    pub foreign_id: String,
    pub title: String,
    pub choice_id: i32,
    pub meta_url: String,
    pub url: String,
    pub thumbnail: Option<String>,
    created_at: NaiveDateTime,
}

impl Question {
    pub fn created_at(&self) -> DateTime<Utc> {
        DateTime::from_utc(self.created_at, Utc)
    }
}

#[derive(Debug, Insertable)]
#[table_name = "answers"]
pub struct NewAnswer<'a> {
    pub ip: &'a ipnetwork::IpNetwork,
    pub question_id: &'a Uuid,
    pub choice_id: i32,
}
