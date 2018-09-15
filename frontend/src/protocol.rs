use std::collections::HashSet;

use capnp::{
    message::{Builder, HeapAllocator, ReaderOptions},
    serialize_packed,
};
use protocol_capnp::{post as Post_P, request, response, update, Vote as Vote_P};

use failure::Error;
use {Post, User, Vote, WsMessage, FetchedPosts, CreatedPost, UsersToUpdate, LoginResponse};

#[derive(Debug, Fail)]
pub enum ProtocolError {
    #[fail(display = "Error response: {}", description)]
    Response { description: String },
}

pub struct ProtocolService {
    data: Vec<u8>,
    builder: Builder<HeapAllocator>,
}

impl ProtocolService {
    pub fn new() -> Self {
        ProtocolService {
            data: Vec::new(),
            builder: Builder::new_default(),
        }
    }

    fn write(&mut self) -> Result<&[u8], Error> {
        self.data.clear();

        serialize_packed::write_message(&mut self.data, &self.builder)?;
        Ok(&self.data)
    }

    pub fn which_message(mut data: &[u8]) -> Result<WsMessage, Error> {
        let reader = serialize_packed::read_message(&mut data, ReaderOptions::new())?;
        let response = reader.get_root::<response::Reader>()?;

        // Check to see first to see if the data is a response message or an update
        let res = match response.which()? {
            response::Login(_) => WsMessage::Login,
            response::Logout(_) => WsMessage::Logout,
            response::FetchPosts(_) => WsMessage::FetchPosts,
            response::CreatePost(_) => WsMessage::CreatePost,
            response::UserVote(_) => WsMessage::UserVote,
            response::Update(data) => match data?.which()? {
                update::Invalid(_) => WsMessage::InvalidPosts,
                update::Users(_) => WsMessage::UpdateUsers,
                update::NewPost(_) => WsMessage::NewPost,
            }
        };

        Ok(res)
    }

    pub fn write_request_login_credentials(
        &mut self,
        name: &str,
        password: &str,
    ) -> Result<&[u8], Error> {
        {
            let mut creds = self
                .builder
                .init_root::<request::Builder>()
                .init_login()
                .init_credentials();
            creds.set_username(name);
            creds.set_password(password);
        }

        self.write()
    }

    pub fn write_request_login_token(&mut self, token: &str) -> Result<&[u8], Error> {
        {
            let mut t = self.builder.init_root::<request::Builder>().init_login();
            t.set_token(token);
        }

        self.write()
    }

    pub fn write_request_logout_token(&mut self, token: &str) -> Result<&[u8], Error> {
        {
            self.builder
                .init_root::<request::Builder>()
                .set_logout(token);
        }

        self.write()
    }

    pub fn write_request_registration(
        &mut self,
        name: &str,
        password: &str,
    ) -> Result<&[u8], Error> {
        {
            let mut registration = self
                .builder
                .init_root::<request::Builder>()
                .init_registration();
            registration.set_username(name);
            registration.set_password(password);
        }
        self.write()
    }

    pub fn write_request_fetch_posts(&mut self, token: &str) -> Result<&[u8], Error> {
        {
            self.builder
                .init_root::<request::Builder>()
                .set_fetch_posts(token);
        }

        self.write()
    }

    pub fn write_request_create_post(
        &mut self,
        token: &str,
        content: &str,
    ) -> Result<&[u8], Error> {
        {
            let mut req = self
                .builder
                .init_root::<request::Builder>()
                .init_create_post();
            req.set_token(token);
            req.set_content(content);
        }

        self.write()
    }

    pub fn write_request_user_vote(
        &mut self,
        token: &str,
        post_id: i32,
        vote: Vote,
    ) -> Result<&[u8], Error> {
        {
            let mut req = self
                .builder
                .init_root::<request::Builder>()
                .init_user_vote();
            let vote = match vote {
                Vote::Up => Vote_P::Up,
                Vote::Down => Vote_P::Down,
                Vote::None => Vote_P::None,
            };

            req.set_token(token);
            req.set_post_id(post_id);
            req.set_vote(vote);
        }

        self.write()
    }

    pub fn read_response_login(&self, mut data: &[u8]) -> Result<Option<LoginResponse>, Error> {
        let reader = serialize_packed::read_message(&mut data, ReaderOptions::new())?;
        let response = reader.get_root::<response::Reader>()?;

        match response.which()? {
            response::Login(data) => match data.which()? {
                response::login::Success(data) => {
                    let token = data.get_token()?.to_string();
                    let user = data.get_user()?;
                    let login_res = LoginResponse {
                        token,
                        user: User {
                            id: user.get_id(),
                            username: user.get_username()?.to_string(),
                            karma: user.get_karma(),
                            streak: user.get_streak(),
                        }
                    };
                    Ok(Some(login_res))
                }
                response::login::Error(error) => Err(Error::from(ProtocolError::Response {
                    description: error?.to_owned(),
                })),
            },
            _ => Ok(None),
        }
    }

    pub fn read_response_logout(&self, mut data: &[u8]) -> Result<Option<()>, Error> {
        let reader = serialize_packed::read_message(&mut data, ReaderOptions::new())?;
        let response = reader.get_root::<response::Reader>()?;

        match response.which()? {
            response::Logout(data) => match data.which()? {
                response::logout::Success(_) => Ok(Some(())),
                response::logout::Error(error) => Err(Error::from(ProtocolError::Response {
                    description: error?.to_owned(),
                })),
            },
            _ => Ok(None),
        }
    }

    pub fn read_response_fetch_posts(
        &self,
        mut data: &[u8],
    ) -> Result<Option<FetchedPosts>, Error> {
        let reader = serialize_packed::read_message(&mut data, ReaderOptions::new())?;
        let response = reader.get_root::<response::Reader>()?;

        match response.which()? {
            response::FetchPosts(data) => match data.which()? {
                response::fetch_posts::Success(data) => {
                    let token = data.get_token()?.to_string();
                    let mut posts = Vec::<Post>::new();

                    for post in data.get_posts()?.iter() {
                        posts.push(Post {
                            id: post.get_id(),
                            content: post.get_content()?.to_string(),
                            valid: post.get_valid(),
                            vote: post.get_vote()?.into(),
                            userId: post.get_user_id(),
                        })
                    }

                    Ok(Some(FetchedPosts { token, posts }))
                }
                response::fetch_posts::Error(error) => Err(Error::from(ProtocolError::Response {
                    description: error?.to_owned(),
                })),
            },
            _ => Ok(None),
        }
    }

    pub fn read_response_create_post(
        &self,
        mut data: &[u8],
    ) -> Result<Option<CreatedPost>, Error> {
        let reader = serialize_packed::read_message(&mut data, ReaderOptions::new())?;
        let response = reader.get_root::<response::Reader>()?;

        match response.which()? {
            response::CreatePost(data) => match data.which()? {
                response::create_post::Success(data) => {
                    let token = data.get_token()?.to_string();
                    let post = data.get_post()?;

                    let post = Post {
                        id: post.get_id(),
                        content: post.get_content()?.to_string(),
                        valid: post.get_valid(),
                        vote: post.get_vote()?.into(),
                        userId: post.get_user_id(),
                    };
                    
                    Ok(Some(CreatedPost { token, post }))
                }
                response::create_post::Error(error) => Err(Error::from(ProtocolError::Response {
                    description: error?.to_owned(),
                })),
            },
            _ => Ok(None),
        }
    }

    pub fn read_request_user_vote(&self, mut data: &[u8]) -> Result<Option<(String)>, Error> {
        let reader = serialize_packed::read_message(&mut data, ReaderOptions::new())?;
        let response = reader.get_root::<response::Reader>()?;

        match response.which()? {
            response::UserVote(data) => match data.which()? {
                response::user_vote::Success(data) => Ok(Some(data?.to_string())),
                response::user_vote::Error(error) => Err(Error::from(ProtocolError::Response {
                    description: error?.to_owned(),
                })),
            },
            _ => Ok(None),
        }
    }

    pub fn read_update_new_post(&self, mut data: &[u8]) -> Result<Option<Post>, Error> {
        let reader = serialize_packed::read_message(&mut data, ReaderOptions::new())?;
        let response = reader.get_root::<response::Reader>()?;

        match response.which()? {
            response::Update(data) => match data?.which()? {
                update::NewPost(data) => {
                    let post = data?;
                    let post = Post {
                        id: post.get_id(),
                        content: post.get_content()?.to_string(),
                        valid: post.get_valid(),
                        vote: post.get_vote()?.into(),
                        userId: post.get_user_id(),
                    };

                    Ok(Some(post))
                },
                _ => Ok(None)
            }
            _ => Ok(None),
        }
    }

    pub fn read_update_invalid(&self, mut data: &[u8]) -> Result<Option<Vec<i32>>, Error> {
        let reader = serialize_packed::read_message(&mut data, ReaderOptions::new())?;
        let response = reader.get_root::<response::Reader>()?;

        match response.which()? {
            response::Update(data) => match data?.which()? {
                update::Invalid(data) => {
                    let post_ids = data?;
                    let mut v = Vec::new();
                    for id in post_ids.iter() {
                        v.push(id);
                    }

                    Ok(Some(v))
                },
                _ => Ok(None)
            }
            _ => Ok(None),
        }
    }

    pub fn read_update_users(&self, mut data: &[u8]) -> Result<Option<UsersToUpdate>, Error> {
        let reader = serialize_packed::read_message(&mut data, ReaderOptions::new())?;
        let response = reader.get_root::<response::Reader>()?;

        match response.which()? {
            response::Update(data) => match data?.which()? {
                update::Users(data) => {
                    let mut users = Vec::new();
                    for user in data?.iter() {
                        users.push(User {
                            id: user.get_id(),
                            username: user.get_username()?.to_string(),
                            karma: user.get_karma(),
                            streak: user.get_streak(),
                        });
                    }

                    Ok(Some(UsersToUpdate { users }))
                },
                _ => Ok(None)
            }
            _ => Ok(None),
        }
    }
}
