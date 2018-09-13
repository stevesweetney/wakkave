extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

extern crate capnp;
extern crate serde;
extern crate wakkave;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate failure;
use failure::Error;

pub use wakkave::protocol_capnp;

use protocol_capnp::{post as Post_P, Vote as Vote_P};

pub mod protocol;
use protocol::ProtocolService;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct ProtocolInterface {
    protocol_builder: ProtocolService
}

#[wasm_bindgen]
impl ProtocolInterface {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        ProtocolInterface {
            protocol_builder: ProtocolService::new(),
        }
    }

    pub fn response_type(&self, bytes: Box<[u8]>) -> WsMessage {
        match ProtocolService::which_message(&bytes) {
            Ok(message) => message,
            Err(e) => {
                log(&e.to_string());
                WsMessage::Error
            }
        }
    }
    
    pub fn read_login(&self, bytes: &[u8]) -> JsValue {
        if let Ok(res) = self.protocol_builder.read_response_login(bytes) {
            JsValue::from_serde(&res.unwrap()).unwrap() 
        } else { JsValue::null() }
    }

    pub fn read_logout(&self, bytes: &[u8]) -> bool {
        if let Ok(Some(())) = self.protocol_builder.read_response_logout(bytes) {
            true
        } else { false }
    }

    pub fn read_fetch_posts(&self, bytes: &[u8]) -> JsValue {
        if let Ok(res) = self.protocol_builder.read_response_fetch_posts(bytes) {
            // returns an instance of FetchedPosts
            JsValue::from_serde(&res.unwrap()).unwrap()
        } else { JsValue::null() }
    }

    pub fn read_create_post(&self, bytes: &[u8]) -> JsValue {
        // returns an instance of CreatedPost
        log("creating a new post through wasm...");
        if let Ok(res) = self.protocol_builder.read_response_create_post(bytes) {
            log(&format!("converting to jsvalue: {:?}", res));
            JsValue::from_serde(&res.unwrap()).unwrap()
        } else { JsValue::null() }
    }

    pub fn read_user_vote(&self, bytes: &[u8]) -> Option<String> {
        if let Ok(res) = self.protocol_builder.read_request_user_vote(bytes) {
            res
        } else { None }
    }

    pub fn read_new_post(&self, bytes: &[u8]) -> JsValue {
        if let Ok(res) = self.protocol_builder.read_update_new_post(bytes) {
            JsValue::from_serde(&res.unwrap()).unwrap()
        } else { JsValue::null() }
    }

    pub fn read_invalid_posts(&self, bytes: &[u8]) -> Option<Box<[i32]>> {
        if let Ok(res) = self.protocol_builder.read_update_invalid(bytes) {
            res.map(|v| v.into_boxed_slice())
        } else { None }
    }

    pub fn read_update_users(&self, bytes: &[u8]) -> JsValue {
        if let Ok(res) = self.protocol_builder.read_update_users(bytes) {
            JsValue::from_serde(&res.unwrap()).unwrap()
        } else { JsValue::null() }
    }

    pub fn write_login_creds(&mut self, name: &str, password: &str) -> Option<Box<[u8]>> {
        if let Ok(res) = self.protocol_builder.write_request_login_credentials(name, password) {
            Some(res.to_vec().into_boxed_slice())
        } else { None }
    }

    pub fn write_login_token(&mut self, token: &str) -> Option<Box<[u8]>> {
        if let Ok(res) = self.protocol_builder.write_request_login_token(token) {
            Some(res.to_vec().into_boxed_slice())
        } else { None }
    }

    pub fn write_logout_token(&mut self, token: &str) -> Option<Box<[u8]>> {
        if let Ok(res) = self.protocol_builder.write_request_logout_token(token) {
            Some(res.to_vec().into_boxed_slice())
        } else { None }
    }

    pub fn write_registration(&mut self, name: &str, password: &str) -> Option<Box<[u8]>> {
        if let Ok(res) = self.protocol_builder.write_request_registration(name, password) {
            Some(res.to_vec().into_boxed_slice())
        } else { None }
    }

    pub fn write_fetch_posts(&mut self, token: &str) -> Option<Box<[u8]>> {
        if let Ok(res) = self.protocol_builder.write_request_fetch_posts(token) {
            Some(res.to_vec().into_boxed_slice())
        } else { None }
    }

    pub fn write_create_post(&mut self, token: &str, content: &str) -> Option<Box<[u8]>> {
        if let Ok(res) = self.protocol_builder.write_request_create_post(token, content) {
            Some(res.to_vec().into_boxed_slice())
        } else { None }
    }

    pub fn write_user_vote(&mut self, token: &str, post_id: i32, vote: u32) -> Option<Box<[u8]>> {
        log("Converting to user vote through wasm");
        let vote = match vote {
            0 => Vote::Up,
            2 => Vote::Down,
            _ => return None,
        };
        if let Ok(res) = self.protocol_builder.write_request_user_vote(token, post_id, vote) {
            log("Succeeded converting vote");
            Some(res.to_vec().into_boxed_slice())
        } else { None }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FetchedPosts {
    token: String,
    posts: Vec<Post>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatedPost {
    token: String,
    post: Post
}

#[derive(Serialize, Deserialize)]
pub struct UsersToUpdate {
    users: Vec<User>
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    token: String,
    user: User
}

#[wasm_bindgen]
pub enum WsMessage {
    Login,
    Logout,
    FetchPosts,
    CreatePost,
    UserVote,
    InvalidPosts,
    NewPost,
    UpdateUsers,
    Error,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    id: i32,
    username: String,
    karma: i32,
    streak: i16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    id: i32,
    content: String,
    valid: bool,
    vote: Vote,
    userId: i32,
}

#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize)]
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

