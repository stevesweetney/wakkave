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

#[cfg(test)]
mod tests {
    use super::*;
    use capnp::{
        message::{Builder, HeapAllocator, ReaderOptions},
        serialize_packed,
    };
    use protocol_capnp::{
        post as Post_P, request, response, update, Vote as Vote_P
    };
    use {
        Post, User, Vote, WsMessage, FetchedPosts, 
        CreatedPost, UsersToUpdate, LoginResponse
    };

    use std::time::SystemTime;

    #[test]
    fn message_type() {
        let protocol_service = ProtocolInterface::new();
        let mut b = Builder::new_default();
        let mut data = Vec::new();
        {
            let update = b.init_root::<response::Builder>().init_update();

            let mut post = update.init_new_post();
            post.set_id(2);
            post.set_content("Hello from steve!");
            post.set_valid(true);
            post.set_user_id(2);
            post.set_vote(Vote_P::None);
        }

        let _ = serialize_packed::write_message(&mut data, &b);
        let boxed_data = data.into_boxed_slice();

        assert_eq!(WsMessage::NewPost, protocol_service.response_type(boxed_data));
    }

    #[test]
    fn message_type_2() {
        let protocol_service = ProtocolInterface::new();
        let mut b = Builder::new_default();
        let mut data = Vec::new();
        {
            let invalid: Vec<i32> = (0..1000).collect();
            let update = b.init_root::<response::Builder>().init_update();

            let mut invalid_posts = update.init_invalid(invalid.len() as u32);

            for (i, n) in invalid.iter().enumerate() {
                invalid_posts.set(i as u32, *n);
            }
        }

        let _ = serialize_packed::write_message(&mut data, &b);
        let boxed_data = data.into_boxed_slice();

        assert_eq!(WsMessage::InvalidPosts, protocol_service.response_type(boxed_data));
    }

    #[test]
    fn message_type_3() {
        let protocol_service = ProtocolInterface::new();
        let mut b = Builder::new_default();
        let mut data = Vec::new();
        {
            let mut success = b
                .init_root::<response::Builder>()
                .init_create_post()
                .init_success();

            success.set_token("new_token");
            let mut p = success.init_post();
            p.set_id(2);
            p.set_content("&post.content");
            p.set_valid(true);
            p.set_user_id(1);
            p.set_vote(Vote_P::None);
        }

        let _ = serialize_packed::write_message(&mut data, &b);
        let boxed_data = data.into_boxed_slice();

        assert_eq!(WsMessage::CreatePost, protocol_service.response_type(boxed_data));
    }
}

