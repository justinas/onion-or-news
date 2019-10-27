use diesel::Insertable;
use uuid::Uuid;

use super::schema::*;

#[derive(Debug, Insertable)]
#[table_name = "questions"]
pub struct NewQuestion<'a> {
    pub id: &'a Uuid,
    pub foreign_id: &'a str,
    pub title: &'a str,
    pub url: &'a str,
    pub choice_id: i32,
}
