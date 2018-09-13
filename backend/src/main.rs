extern crate dotenv;
extern crate backend;

use dotenv::dotenv;
use backend::server::Server;

fn main() {
    dotenv().ok();

    let server = Server::new();

    server.start();
}
