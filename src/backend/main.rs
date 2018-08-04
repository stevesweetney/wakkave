extern crate dotenv;
extern crate wakkave;

use dotenv::dotenv;
use wakkave::backend::server::Server;

fn main() {
    dotenv().ok();

    let server = Server::new();

    server.start();
}