use super::schema::{posts, sessions, users, votes};
use std::time::SystemTime;

#[derive(Queryable, Insertable)]
#[table_name = "sessions"]
pub struct Session {
    pub id: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub karma: i32,
    pub streak: i16,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost {
    pub content: String,
    pub user_id: i32,
}

#[derive(Queryable, Identifiable)]
pub struct Post {
    pub id: i32,
    pub content: String,
    pub valid: bool,
    pub created_at: SystemTime,
    pub user_id: i32,
}

#[derive(Queryable, Insertable, Identifiable, Associations)]
#[primary_key(user_id, post_id)]
#[belongs_to(Post)]
#[table_name = "votes"]
pub struct Vote {
    pub post_id: i32,
    pub user_id: i32,
    pub up_or_down: i16,
}
