mod types;

use std::env;
use std::net::SocketAddr;
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

async fn insert_answer(
    db: Arc<Database>,
    ip: std::net::IpAddr,
    question_id: uuid::Uuid,
    choice_id: i32,
) -> Result<usize, oon_db::Error> {
    tokio_executor::blocking::run(move || db.insert_answer(ip, question_id, choice_id)).await
}

async fn guess_handler(
    db: Arc<Database>,
    request: GuessRequest,
    remote_addr: std::net::IpAddr,
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

    if let (Some(qid), Some(cid)) = (this_question.map(|q| q.id), request.choice_id) {
        let ip = remote_addr;
        insert_answer(db.clone(), ip, qid.clone(), cid)
            .await
            .map_err(warp::reject::custom)?;
    }

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

/// Gets the remote client IP.
///
/// Note: the server MUST be behind a reverse proxy
/// that sets the X-Forwarded-For header.
/// Else, the client might spoof the header.
///
/// The fallback on remote socket address can be useful in development.
fn get_ip() -> impl Filter<Extract = (std::net::IpAddr,), Error = warp::Rejection> + Clone {
    warp::header("x-forwarded-for")
        .or(warp::addr::remote().map(|a: Option<SocketAddr>| a.expect("No remote addr").ip()))
        .unify()
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
        .and(get_ip())
        .and_then(guess_handler);

    let index = warp::path::end().and(warp::fs::file("static/index.html"));

    let script = warp::path("script.js").and(warp::fs::file("static/script.js"));

    warp::serve(index.or(script).or(guess))
        .run(([127u8, 0, 0, 1], port))
        .await;
    Ok(())
}
