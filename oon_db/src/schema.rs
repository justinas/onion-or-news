table! {
    answers (id) {
        id -> Int8,
        ip -> Inet,
        question_id -> Uuid,
        choice_id -> Int4,
    }
}

table! {
    questions (id) {
        id -> Uuid,
        foreign_id -> Text,
        title -> Text,
        choice_id -> Int4,
        meta_url -> Text,
        url -> Text,
        thumbnail -> Nullable<Text>,
    }
}

joinable!(answers -> questions (question_id));

allow_tables_to_appear_in_same_query!(
    answers,
    questions,
);
