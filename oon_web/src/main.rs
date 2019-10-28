mod types;

use std::env;
use std::sync::Arc;

use warp::Filter;

use oon_db::{models, Database};

use types::*;

async fn get_question(
    db: Arc<Database>,
    id: uuid::Uuid,
) -> Result<models::Question, oon_db::Error> {
    tokio_executor::blocking::run(move || db.get_question(id)).await
}

async fn get_random_question(db: Arc<Database>) -> Result<models::Question, oon_db::Error> {
    tokio_executor::blocking::run(move || db.get_random_question()).await
}

async fn guess_handler(
    db: Arc<Database>,
    request: GuessRequest,
) -> Result<impl warp::Reply, warp::reject::Rejection> {
    let this_question = match request.question_id {
        Some(id) => Some(
            get_question(db.clone(), id)
                .await
                .map_err(warp::reject::custom)?,
        ),
        None => None,
    };
    let this_question = this_question.as_ref();

    let next_question = get_random_question(db)
        .await
        .map_err(warp::reject::custom)?;

    Ok(warp::reply::json(&GuessResponse {
        correct_choice_id: this_question.map(|q| q.choice_id),
        your_choice_id: request.choice_id,
        url: this_question.map(|q| q.url.clone()),
        meta_url: this_question.map(|q| q.meta_url.clone()),
        thumbnail: this_question.and_then(|q| q.thumbnail.clone()),

        next_question_id: next_question.id,
        next_question_title: next_question.title,
    }))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let port = if let Ok(p) = env::var("PORT") {
        p.parse()?
    } else {
        8000u16
    };

    let db_url = env::var("DATABASE_URL").expect("Expected DATABASE_URL to be set");
    let db_arc = Arc::new(Database::connect(&db_url)?);
    db_arc.migrate()?;

    let base = warp::any()
        .and(warp::body::content_length_limit(2 << 12))
        .map(move || db_arc.clone());

    let guess = base
        .and(warp::path("guess"))
        .and(warp::body::json::<GuessRequest>())
        .and_then(guess_handler);

    let index = warp::path::end()
        .and(warp::fs::file("static/index.html"));

    let script = warp::path("script.js")
        .and(warp::fs::file("static/script.js"));

    warp::serve(index.or(script).or(guess)).run(([127u8, 0, 0, 1], port)).await;
    Ok(())
}
