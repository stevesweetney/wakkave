use super::schema::{sessions, users};

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
