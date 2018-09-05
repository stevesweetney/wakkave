use protocol_capnp::{post as Post_P, Vote as Vote_P};

pub mod components;
pub mod routes;
pub mod services;

pub const SESSION_TOKEN: &str = "SessionToken";

pub enum WsMessage {
    Login,
    Logout,
    FetchPosts,
    CreatePost,
    UserVote,
    InvalidPosts,
    NewPost,
    UpdateUsers,
}

pub struct User {
    id: i32,
    username: String,
    karma: i32,
    streak: i16,
}

pub struct Post {
    id: i32,
    content: String,
    valid: bool,
    vote: Vote,
    userId: i32,
}

pub enum Vote {
    Up,
    None,
    Down,
}

impl Into<Vote_P> for Vote {
    fn into(self) -> Vote_P {
        match self {
            Vote::None => Vote_P::None,
            Vote::Up => Vote_P::Up,
            Vote::Down => Vote_P::Down,
        }
    }
}

impl From<Vote_P> for Vote {
    fn from(v: Vote_P) -> Self {
        match v {
            Vote_P::None => Vote::None,
            Vote_P::Up => Vote::Up,
            Vote_P::Down => Vote::Down,
        }
    }
}

#[derive(Debug, Fail)]
#[fail(display = "Invalid Request")]
pub struct InvalidRequest;
