use actix::{prelude::*, SystemRunner};
use actix_web::{ ws, server, App, HttpRequest, HttpResponse, Error }; 
use super::{websocket::Ws, database::executor::DbExecutor, chatserver::ChatServer};
use super::State;
use diesel::{prelude::*, r2d2::ConnectionManager};
use r2d2::Pool;

use std::env;

pub struct Server {
    runner: SystemRunner,
}

fn create_user(_req: &HttpRequest<State>) -> & 'static str {
    "Thank you for creating a user!"
}

fn connect_ws(req: &HttpRequest<State>) -> Result<HttpResponse, Error> {
    ws::start(req, Ws::default())
}

impl Server {
    pub fn new() -> Self {
        let runner = actix::System::new("Wakkave Server");

        let database_url = env::var("DATABASE_URL")
            .expect("Expected a database url to be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder().build(manager)
            .expect("Failed to create pool");
        let db_addr = SyncArbiter::start(1, move || DbExecutor(pool.clone()));
        let chat_addr = Arbiter::start(|_| ChatServer::default());

        server::new(move || App::with_state(State { db: db_addr.clone(), chat: chat_addr.clone() })
            .resource("/", |r| r.f(connect_ws))
            .resource("/users/new", |r| r.f(create_user)))
                .bind("127.0.0.1:8088")
                .unwrap()
                .start();

        Server { runner }
    }

    pub fn start(self) -> i32 {
        self.runner.run()
    }
}