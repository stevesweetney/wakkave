use super::{
    chatserver::ChatServer,
    database::executor::{
        CreatePost, CreateSession, CreateUser, DbExecutor, DeleteSession, FetchPosts, FindUser,
        FindUserID, UpdateSession, UserVote,
    },
    token::Token,
    websocket::Ws,
    State,
};
use actix::{prelude::*, SystemRunner};
use actix_web::{
    error, fs::StaticFiles, http, server, ws, App, AsyncResponder, Error, FutureResponse,
    HttpMessage, HttpRequest, HttpResponse, Responder,
};
use bytes::Bytes;
use capnp::{
    self,
    message::{Builder, HeapAllocator, ReaderOptions},
    serialize_packed, text,
};
use diesel::{prelude::*, r2d2::ConnectionManager};
use failure::Error as FailError;
use futures::Future;
use protocol_capnp::{request, response, Vote};
use r2d2::Pool;
use std::env;

pub struct Server {
    runner: SystemRunner,
}

fn handle_request_login_token(
    data: Result<text::Reader, capnp::Error>,
    db: Addr<DbExecutor>,
) -> Result<Vec<u8>, FailError> {
    let token = data?;
    println!("Renewing Token: {} \n", token);

    let (new_id, user_id) = Token::verify(token)?;

    let new_token = db
        .send(UpdateSession {
            old_id: token.to_string(),
            new_id,
        }).wait()??;

    let user = db.send(FindUserID { user_id }).wait()??;

    match user {
        Some(user) => {
            let mut builder = Builder::new_default();
            {
                let mut success = builder
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

            let mut res = Vec::new();
            serialize_packed::write_message(&mut res, &builder)?;
            Ok(res)
        }
        None => Err(super::ServerError::FindUser.into()),
    }
}

fn handle_request_login_credentials(
    data: request::login::credentials::Reader,
    db: Addr<DbExecutor>,
) -> Result<Vec<u8>, FailError> {
    let name = data.get_username()?;
    let password = data.get_password()?;
    println!("Name: {} \nPassword: {}", name, password);

    let user = db
        .send(FindUser {
            username: name.to_string(),
            password: password.to_string(),
        }).wait()??;

    match user {
        Some(user) => {
            let token = db
                .send(CreateSession {
                    id: Token::create(user.id)?,
                }).wait()??;
            let mut builder = Builder::new_default();
            {
                let mut success = builder
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

            let mut res = Vec::new();
            serialize_packed::write_message(&mut res, &builder)?;
            Ok(res)
        }
        None => Err(super::ServerError::FindUser.into()),
    }
}

fn handle_request_registration(
    data: request::registration::Reader,
    db: Addr<DbExecutor>,
) -> Result<Vec<u8>, FailError> {
    let username = data.get_username()?.to_string();
    let password = data.get_password()?.to_string();
    let user = db.send(CreateUser { username, password }).wait()??;
    let res = {
        let mut builder = Builder::new_default();
        {
            let mut success = builder
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
        let mut res = Vec::new();
        serialize_packed::write_message(&mut res, &builder)?;
        res
    };

    Ok(res)
}

fn login_register(req: &HttpRequest<State>) -> FutureResponse<Bytes> {
    let db = req.state().db.clone();
    req.body() // <- get Body future
        .from_err()
        .and_then(|bytes: Bytes| {
            let reader = serialize_packed::read_message(&mut bytes.as_ref(), ReaderOptions::new())
                .expect("Error reading message");

            let request = reader
                .get_root::<request::Reader>()
                .expect("Error getting message root");

            match request.which() {
                Ok(request::Login(data)) => match data.which() {
                    Ok(request::login::Credentials(data)) => {
                        match handle_request_login_credentials(data, db) {
                            Ok(res) => Ok(Bytes::from(res)), //self.connect_to_chat(ctx),
                            Err(e) => {
                                let mut builder = Builder::new_default();
                                builder
                                    .init_root::<response::Builder>()
                                    .init_login()
                                    .set_error(&e.to_string());
                                let mut error_res = Vec::new();
                                serialize_packed::write_message(&mut error_res, &builder)
                                    .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;
                                println!("{}", &e.to_string());
                                Ok(Bytes::from(error_res))
                            }
                        }
                    }
                    Ok(request::login::Token(data)) => match handle_request_login_token(data, db) {
                        Ok(res) => Ok(Bytes::from(res)),
                        Err(e) => {
                            let mut builder = Builder::new_default();
                            builder
                                .init_root::<response::Builder>()
                                .init_login()
                                .set_error(&e.to_string());
                            let mut error_res = Vec::new();
                            serialize_packed::write_message(&mut error_res, &builder)?;

                            Ok(Bytes::from(error_res))
                        }
                    },
                    Err(::capnp::NotInSchema(_)) => {
                        Err(error::ErrorInternalServerError("Invalid data"))
                    }
                },
                Ok(request::Registration(data)) => match handle_request_registration(data, db) {
                    Ok(res) => Ok(Bytes::from(res)),
                    Err(e) => {
                        let mut builder = Builder::new_default();
                        builder
                            .init_root::<response::Builder>()
                            .init_login()
                            .set_error(&e.to_string());
                        let mut error_res = Vec::new();
                        serialize_packed::write_message(&mut error_res, &builder)?;

                        Ok(Bytes::from(error_res))
                    }
                },
                _ => Err(error::ErrorInternalServerError("Invalid data")),
            }
        }).responder()
}

fn connect_ws(req: &HttpRequest<State>) -> Result<HttpResponse, Error> {
    ws::start(req, Ws::default())
}

impl Server {
    pub fn new() -> Self {
        embed_migrations!();
        let runner = actix::System::new("Wakkave Server");

        let database_url = env::var("DATABASE_URL").expect("Expected a database url to be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
            .build(manager)
            .expect("Failed to create pool");
        embedded_migrations::run(&pool.get().unwrap());
        let db_addr = SyncArbiter::start(1, move || DbExecutor(pool.clone()));
        let db_clone = db_addr.clone();
        let chat_addr = Arbiter::start(move |_| ChatServer::new(db_clone));

        server::new(move || {
            App::with_state(State {
                db: db_addr.clone(),
                chat: chat_addr.clone(),
            }).resource("/ws/", |r| r.f(connect_ws))
            .resource("/login", |r| r.method(http::Method::POST).f(login_register))
            .default_resource(|r| r.h(http::NormalizePath::default()))
            .handler(
                "/",
                StaticFiles::new("./static")
                    .unwrap()
                    .index_file("index.html"),
            )
        }).bind("0.0.0.0:80")
        .unwrap()
        .start();

        Server { runner }
    }

    pub fn start(self) -> i32 {
        self.runner.run()
    }
}
