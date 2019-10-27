mod types;

use std::env;
use std::sync::Arc;

use oon_db::{models, Database};

use types::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_url = env::var("DATABASE_URL").expect("Expected DATABASE_URL to be set");
    let db = Arc::new(Database::connect(&db_url)?);

    let list: Listing = reqwest::get("https://reddit.com/r/theonion.json")
        .await?
        .json()
        .await?;

    let inserts = list.data.children.into_iter().map(|post| {
        let db = db.clone();
        tokio_executor::blocking::run(move || {
            let id = uuid::Uuid::new_v4();
            let question = models::NewQuestion {
                id: &id,
                foreign_id: &post.data.name,
                title: &post.data.title,
                url: &post.data.full_permalink(),
                choice_id: 1,
            };
            db.insert_question(&question).unwrap();
        })
    });

    futures::future::join_all(inserts).await;

    Ok(())
}
