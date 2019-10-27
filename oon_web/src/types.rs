use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct GuessRequest {
    pub choice_id: Option<i32>,
    pub question_id: Option<uuid::Uuid>,
}

#[derive(Serialize)]
pub struct GuessResponse {
    pub correct_choice_id: Option<i32>,
    pub your_choice_id: Option<i32>,
    pub next_question_id: uuid::Uuid,
    pub next_question_title: String,
}
