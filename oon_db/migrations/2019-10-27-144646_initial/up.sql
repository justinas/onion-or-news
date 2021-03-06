CREATE TABLE questions (
    id          UUID PRIMARY KEY,
    foreign_id  TEXT NOT NULL,
    title       TEXT NOT NULL,
    choice_id   INT NOT NULL,
    meta_url    TEXT NOT NULL,
    url         TEXT NOT NULL,
    thumbnail   TEXT
);

CREATE UNIQUE INDEX ON questions (foreign_id);
CREATE INDEX on questions (choice_id);

CREATE TABLE answers (
    id          BIGSERIAL PRIMARY KEY,
    ip          INET NOT NULL,
    question_id UUID NOT NULL REFERENCES questions(id),
    choice_id   INT NOT NULL
);

CREATE UNIQUE INDEX ON answers (ip, question_id);
CREATE INDEX ON answers (question_id);
