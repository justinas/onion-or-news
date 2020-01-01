#![deny(clippy::all)]

mod types;

use std::env;
use std::net::SocketAddr;
use std::sync::Arc;

use log::error;
use tokio::task;
use warp::reject::LengthRequired;
use warp::Filter;

use oon_db::{models, Database};

use types::*;

#[derive(Debug)]
enum Error {
    Database(oon_db::Error),
}

impl warp::reject::Reject for Error {}

impl From<oon_db::Error> for Error {
    fn from(e: oon_db::Error) -> Self {
        Error::Database(e)
    }
}

async fn get_question(db: Arc<Database>, id: uuid::Uuid) -> Result<models::Question, Error> {
    // TODO: error-handling
    task::spawn_blocking(move || db.get_question(id))
        .await
        .unwrap()
        .map_err(Into::into)
}

async fn get_random_question(db: Arc<Database>) -> Result<models::Question, Error> {
    // TODO: error-handling
    task::spawn_blocking(move || db.get_random_question())
        .await
        .unwrap()
        .map_err(Into::into)
}

async fn insert_answer(
    db: Arc<Database>,
    ip: std::net::IpAddr,
    question_id: uuid::Uuid,
    choice_id: i32,
) -> Result<usize, Error> {
    // TODO: error-handling
    task::spawn_blocking(move || db.insert_answer(ip, question_id, choice_id))
        .await
        .unwrap()
        .map_err(Into::into)
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
        insert_answer(db.clone(), ip, qid, cid)
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
fn get_ip() -> impl Filter<Extract = (std::net::IpAddr,), Error = std::convert::Infallible> + Clone
{
    warp::header("x-forwarded-for")
        .or(warp::addr::remote().map(|a: Option<SocketAddr>| a.expect("No remote addr").ip()))
        .unify()
}

/// Hides errors from users, responding with a generic status code.
async fn recover(rejection: warp::Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    if rejection.is_not_found() {
        Err(rejection)
    } else if rejection.find::<LengthRequired>().is_some() {
        Ok(warp::reply::with_status(
            warp::reply(),
            warp::http::StatusCode::BAD_REQUEST,
        ))
    } else {
        error!("{:?}", rejection);
        Ok(warp::reply::with_status(
            warp::reply(),
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
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

    let guess = warp::path("guess")
        .and(warp::body::content_length_limit(2 << 12))
        .map(move || db_arc.clone())
        .and(warp::body::json::<GuessRequest>())
        .and(get_ip())
        .and_then(guess_handler);

    let index = warp::path::end().and(warp::fs::file("static/index.html"));

    let script = warp::path("script.js").and(warp::fs::file("static/script.js"));

    let routes = index.or(script).or(guess).recover(recover);

    warp::serve(routes).run(([127u8, 0, 0, 1], port)).await;
    Ok(())
}
