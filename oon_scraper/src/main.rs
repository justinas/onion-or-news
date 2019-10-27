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
                    url: &post.data.full_permalink(),
                    choice_id: match &*subreddit {
                        "theonion" => 1,
                        "nottheonion" => 2,
                        _ => panic!("Unexpected subreddit: {}", post.data.subreddit),
                    },
                };
                db.insert_question(&question).unwrap();
                println!("Inserted {:?}", question.title);
            })
        });

        futures::future::join_all(inserts).await;

        after = list.data.after;
        if after.is_none() {
            break;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_url = env::var("DATABASE_URL").expect("Expected DATABASE_URL to be set");
    let db = Arc::new(Database::connect(&db_url)?);

    let (r1, r2) = futures::future::join(
        get_all(db.clone(), "theonion"),
        get_all(db.clone(), "nottheonion"),
    )
    .await;
    r1?;
    r2?;
    Ok(())
}
