-- To be able to select an exact number of random rows
CREATE EXTENSION tsm_system_rows;

CREATE TABLE questions (
    id          UUID PRIMARY KEY,
    foreign_id  TEXT NOT NULL,
    title       TEXT NOT NULL,
    url         TEXT NOT NULL,
    choice_id   INT NOT NULL
);

CREATE UNIQUE INDEX ON questions (foreign_id);

CREATE TABLE answers (
    id          BIGINT PRIMARY KEY,
    ip          INET NOT NULL,
    question_id UUID NOT NULL REFERENCES questions(id),
    choice_id   INT NOT NULL
);

CREATE UNIQUE INDEX ON answers (ip, id);
CREATE INDEX ON answers (question_id);
