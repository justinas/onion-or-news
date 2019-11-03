#![deny(clippy::all)]

mod types;

use std::env;
use std::sync::Arc;

use itertools::Itertools;

use oon_db::{models, Database};

use types::*;

// The Onion uses a weird title-case where they capitalize each word,
// even ones like "a", "an", "in", etc.
//
// In order to make the game harder,
// we try to convert everything into this weird title case.
fn bad_title_case(s: &str) -> String {
    s.split_whitespace()
        .map(|word| {
            let mut new_word = String::with_capacity(s.len());
            let mut chars = word.chars();
            for c in chars.next().iter().copied().flat_map(char::to_uppercase) {
                new_word.push(c);
            }
            for c in chars {
                new_word.push(c);
            }
            new_word
        })
        .intersperse(String::from(" "))
        .collect()
}

async fn get(
    subreddits: &[&str],
    after: Option<&str>,
) -> Result<Listing, Box<dyn std::error::Error>> {
    let mut url = format!(
        "https://reddit.com/r/{}.json?sort=top",
        subreddits.join("+")
    );
    if let Some(s) = after {
        url.push_str("&after=");
        url.push_str(s);
    }
    Ok(reqwest::get(&url).await?.json().await?)
}

async fn get_all(db: Arc<Database>, subreddit: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut after: Option<String> = None;
    let mut skipped = 0;
    loop {
        let list = get(&[subreddit], after.as_ref().map(String::as_ref)).await?;

        let inserts = list.data.children.into_iter().map(|post| {
            let db = db.clone();
            tokio_executor::blocking::run(move || {
                let id = uuid::Uuid::new_v4();
                let subreddit = post.data.subreddit.to_ascii_lowercase();
                let title = bad_title_case(&post.data.title);
                let question = models::NewQuestion {
                    id: &id,
                    foreign_id: &post.data.name,
                    title: &title,
                    choice_id: match &*subreddit {
                        "theonion" => 1,
                        "nottheonion" => 2,
                        _ => panic!("Unexpected subreddit: {}", post.data.subreddit),
                    },
                    meta_url: &post.data.full_permalink(),
                    url: &post.data.url,
                    thumbnail: post.data.thumbnail.as_ref().map(String::as_ref),
                };
                let affected_rows = db.insert_question(&question).unwrap();
                if affected_rows == 1 {
                    println!("Inserted {:?}", question.title);
                }
                affected_rows
            })
        });

        let results = futures::future::join_all(inserts).await;
        skipped += results.iter().filter(|&&c| c == 0).count();

        after = list.data.after;
        if after.is_none() {
            break;
        }
    }

    println!("Skipped {} existing entries", skipped);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_url = env::var("DATABASE_URL").expect("Expected DATABASE_URL to be set");
    let db = Arc::new(Database::connect(&db_url)?);
    db.migrate()?;

    let (r1, r2) = futures::future::join(
        get_all(db.clone(), "theonion"),
        get_all(db.clone(), "nottheonion"),
    )
    .await;
    r1?;
    r2?;
    Ok(())
}
