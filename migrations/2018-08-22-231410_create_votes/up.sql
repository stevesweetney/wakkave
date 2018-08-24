-- Your SQL goes here
CREATE TABLE votes (
    user_id INTEGER NOT NULL REFERENCES users (id),
    post_id INTEGER NOT NULL REFERENCES posts (id),
    up_or_down SMALLINT NOT NULL,
    PRIMARY KEY (user_id, post_id) 
)