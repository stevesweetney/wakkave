use actix::{fut, prelude::*};
use actix_web::{
    ws::{Message, ProtocolError, WebsocketContext},
    Binary,
};

use capnp::{
    self,
    message::{Builder, HeapAllocator, ReaderOptions},
    serialize_packed, text,
};

use backend::{
    chatserver,
    database::executor::{
        CreatePost, CreateSession, CreateUser, DeleteSession, FetchPosts, FindUser, FindUserID,
        UpdateSession, UserVote,
    },
    token::Token,
    State,
};

use protocol_capnp::{request, response, Vote};

use std::default::Default;

use failure::Error;
use futures::future::Future;

pub struct Ws {
    data: Vec<u8>,
    builder: Builder<HeapAllocator>,
    id: Option<String>,
}

impl Default for Ws {
    fn default() -> Self {
        Self::new()
    }
}

impl Actor for Ws {
    type Context = WebsocketContext<Self, State>;

    fn stopping(&mut self, ctx: &mut Self::Context) -> Running {
        // notify the chat server
        if let Some(ref id) = self.id {
            ctx.state()
                .chat
                .do_send(chatserver::Disconnect { id: id.to_owned() });
        }

        Running::Stop
    }
}

/// Handle messages from chat server, we simply send it to peer websocket
impl Handler<chatserver::ServerMessage> for Ws {
    type Result = ();

    fn handle(&mut self, msg: chatserver::ServerMessage, ctx: &mut Self::Context) {
        ctx.binary(msg.0);
    }
}

impl StreamHandler<Message, ProtocolError> for Ws {
    fn handle(&mut self, msg: Message, ctx: &mut Self::Context) {
        match msg {
            Message::Text(text) => {
                ctx.text(text);
            }
            Message::Binary(bin) => {
                self.handle_request(&bin, ctx);
            }
            Message::Close(_reason) => {
                ctx.stop();
            }
            _ => (),
        };
    }
}

impl Ws {
    pub fn new() -> Self {
        Ws {
            data: Vec::new(),
            builder: Builder::new_default(),
            id: None,
        }
    }

    fn handle_request(&mut self, data: &Binary, ctx: &mut WebsocketContext<Self, State>) {
        let reader = serialize_packed::read_message(&mut data.as_ref(), ReaderOptions::new())
            .expect("Error reading message");

        let request = reader
            .get_root::<request::Reader>()
            .expect("Error getting message root");

        match request.which() {
            Ok(request::Login(data)) => match data.which() {
                Ok(request::login::Credentials(data)) => {
                    match self.handle_request_login_credentials(data, ctx) {
                        Ok(()) => self.connect_to_chat(ctx),
                        Err(e) => {
                            self.builder
                                .init_root::<response::Builder>()
                                .init_login()
                                .set_error(&e.to_string());
                            println!("Error: {:?}", e);
                        }
                    }

                    self.send(ctx);
                }
                Ok(request::login::Token(data)) => {
                    match self.handle_request_login_token(data, ctx) {
                        Ok(()) => self.connect_to_chat(ctx),
                        Err(e) => {
                            self.builder
                                .init_root::<response::Builder>()
                                .init_login()
                                .set_error(&e.to_string());
                            let _ = self.write();
                            println!("Error: {:?}", e);
                        }
                    }

                    self.send(ctx);
                }
                Err(::capnp::NotInSchema(_)) => (),
            },
            Ok(request::Registration(data)) => {
                match self.handle_request_registration(data, ctx) {
                    Ok(()) => self.connect_to_chat(ctx),
                    Err(e) => {
                        self.builder
                            .init_root::<response::Builder>()
                            .init_login()
                            .set_error(&e.to_string());
                        let _ = self.write();
                    }
                }

                self.send(ctx);
            }
            Ok(request::Logout(data)) => {
                if let Err(e) = self.handle_request_logout(data, ctx) {
                    self.builder
                        .init_root::<response::Builder>()
                        .init_logout()
                        .set_error(&e.to_string());
                    let _ = self.write();
                }

                self.send(ctx);
            }
            Ok(request::FetchPosts(data)) => {
                match self.handle_request_fetch_posts(data, ctx) {
                    Ok(()) => (),
                    Err(e) => {
                        self.builder
                            .init_root::<response::Builder>()
                            .init_fetch_posts()
                            .set_error(&e.to_string());
                        let _ = self.write();
                    }
                }

                self.send(ctx);
            }
            Ok(request::CreatePost(data)) => {
                match self.handle_request_create_post(data, ctx) {
                    Ok(()) => (),
                    Err(e) => {
                        self.builder
                            .init_root::<response::Builder>()
                            .init_create_post()
                            .set_error(&e.to_string());
                        let _ = self.write();
                    }
                }

                self.send(ctx);
            }
            Ok(request::UserVote(data)) => {
                match self.handle_request_user_vote(data, ctx) {
                    Ok(()) => (),
                    Err(e) => {
                        self.builder
                            .init_root::<response::Builder>()
                            .init_user_vote()
                            .set_error(&e.to_string());
                        let _ = self.write();
                    }
                }

                self.send(ctx);
            }
            Err(::capnp::NotInSchema(_)) => (),
        }
    }

    fn write(&mut self) -> Result<(), Error> {
        self.data.clear();

        serialize_packed::write_message(&mut self.data, &self.builder)?;
        Ok(())
    }

    fn send(&self, ctx: &mut WebsocketContext<Self, State>) {
        ctx.binary(self.data.clone());
    }

    fn connect_to_chat(&self, ctx: &mut WebsocketContext<Self, State>) {
        let addr = ctx.address();
        ctx.state()
            .chat
            .send(chatserver::Connect {
                addr: addr.recipient(),
            }).into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = Some(res),
                    // something is wrong with chat server
                    _ => ctx.stop(),
                }
                fut::ok(())
            }).wait(ctx);
    }

    fn handle_request_login_credentials(
        &mut self,
        data: request::login::credentials::Reader,
        ctx: &mut WebsocketContext<Self, State>,
    ) -> Result<(), Error> {
        let name = data.get_username()?;
        let password = data.get_password()?;
        println!("Name: {} \nPassword: {}", name, password);

        let user = ctx
            .state()
            .db
            .send(FindUser {
                username: name.to_string(),
                password: password.to_string(),
            }).wait()??;

        match user {
            Some(user) => {
                let token = ctx
                    .state()
                    .db
                    .send(CreateSession {
                        id: Token::create(user.id)?,
                    }).wait()??;

                let mut success = self
                    .builder
                    .init_root::<response::Builder>()
                    .init_login()
                    .init_success();

                success.set_token(&token.id);

                let mut u = success.init_user();
                u.set_id(user.id);
                u.set_username(&user.username);
                u.set_karma(user.karma);
                u.set_streak(user.streak);
            }
            None => {
                return Err(super::ServerError::FindUser.into());
            }
        }

        self.write()
    }

    fn handle_request_login_token(
        &mut self,
        data: Result<text::Reader, capnp::Error>,
        ctx: &mut WebsocketContext<Self, State>,
    ) -> Result<(), Error> {
        let token = data?;
        println!("Renewing Token: {} \n", token);

        let (new_id, user_id) = Token::verify(token)?;

        let new_token = ctx
            .state()
            .db
            .send(UpdateSession {
                old_id: token.to_string(),
                new_id,
            }).wait()??;

        let user = ctx.state().db.send(FindUserID { user_id }).wait()??;

        match user {
            Some(user) => {
                let mut success = self
                    .builder
                    .init_root::<response::Builder>()
                    .init_login()
                    .init_success();

                success.set_token(&new_token.id);

                let mut u = success.init_user();
                u.set_id(user.id);
                u.set_username(&user.username);
                u.set_karma(user.karma);
                u.set_streak(user.streak);
            }
            None => {
                return Err(super::ServerError::FindUser.into());
            }
        }

        self.write()
    }

    fn handle_request_registration(
        &mut self,
        data: request::registration::Reader,
        ctx: &mut WebsocketContext<Self, State>,
    ) -> Result<(), Error> {
        let username = data.get_username()?.to_string();
        let password = data.get_password()?.to_string();
        let user = ctx
            .state()
            .db
            .send(CreateUser { username, password })
            .wait()??;
        {
            let mut success = self
                .builder
                .init_root::<response::Builder>()
                .init_login()
                .init_success();
            success.set_token(&Token::create(user.id)?);

            let mut u = success.init_user();
            u.set_id(user.id);
            u.set_username(&user.username);
            u.set_karma(user.karma);
            u.set_streak(user.streak);
        }

        self.write()
    }

    fn handle_request_logout(
        &mut self,
        data: Result<text::Reader, capnp::Error>,
        ctx: &mut WebsocketContext<Self, State>,
    ) -> Result<(), Error> {
        let token = data?;
        ctx.state()
            .db
            .send(DeleteSession {
                session_id: token.to_string(),
            }).wait()??;

        self.builder
            .init_root::<response::Builder>()
            .init_logout()
            .set_success(());
        self.write()
    }

    fn handle_request_fetch_posts(
        &mut self,
        data: Result<text::Reader, capnp::Error>,
        ctx: &mut WebsocketContext<Self, State>,
    ) -> Result<(), Error> {
        let token = data?;
        let (new_token, user_id) = Token::verify(token)?;
        let res = ctx.state().db.send(FetchPosts { user_id }).wait()??;
        ctx.state().db.do_send(UpdateSession {
            old_id: token.to_string(),
            new_id: new_token.clone(),
        });

        {
            let mut success = self
                .builder
                .init_root::<response::Builder>()
                .init_fetch_posts()
                .init_success();
            success.set_token(&new_token);
            let mut fetched_posts = success.init_posts(res.len() as u32);

            for (i, (post, vote)) in res.iter().enumerate() {
                let mut p = fetched_posts.reborrow().get(i as u32);
                p.set_id(post.id);
                p.set_content(&post.content);
                p.set_valid(post.valid);
                p.set_user_id(post.user_id);
                let vote = match vote {
                    None => Vote::None,
                    Some(v) => match v.up_or_down {
                        1 => Vote::Up,
                        -1 => Vote::Down,
                        _ => Vote::None,
                    },
                };
                p.set_vote(vote);
            }
        }
        self.write()
    }

    fn handle_request_create_post(
        &mut self,
        data: request::create_post::Reader,
        ctx: &mut WebsocketContext<Self, State>,
    ) -> Result<(), Error> {
        let token = data.get_token()?;
        let content = data.get_content()?.to_string();

        let (new_token, user_id) = Token::verify(token)?;
        let (post, _username) = ctx
            .state()
            .db
            .send(CreatePost { user_id, content })
            .wait()??;

        {
            let mut success = self
                .builder
                .init_root::<response::Builder>()
                .init_create_post()
                .init_success();

            success.set_token(&new_token);
            let mut p = success.init_post();
            p.set_id(post.id);
            p.set_content(&post.content);
            p.set_valid(post.valid);
            p.set_user_id(post.user_id);
            p.set_vote(Vote::None);
        }

        if let Some(ref id) = self.id {
            ctx.state().chat.do_send(chatserver::ClientMessage {
                id: id.to_owned(),
                msg: post,
            });
        }

        self.write()
    }

    fn handle_request_user_vote(
        &mut self,
        data: request::user_vote::Reader,
        ctx: &mut WebsocketContext<Self, State>,
    ) -> Result<(), Error> {
        let token = data.get_token()?;
        let vote = data.get_vote()?;
        let post_id = data.get_post_id();

        let (new_token, user_id) = Token::verify(token)?;
        let up_or_down = match vote {
            Vote::Up => 1,
            Vote::Down => 0,
            Vote::None => return Err(super::ServerError::InvalidVote.into()),
        };

        let _ = ctx.state().db.send(UserVote {
            post_id,
            user_id,
            up_or_down,
        });

        {
            self.builder
                .init_root::<response::Builder>()
                .init_user_vote()
                .set_success(&new_token);
        }

        self.write()
    }
}
