extern crate backend;
extern crate dotenv;

use backend::server::Server;
use dotenv::dotenv;

fn main() {
    dotenv().ok();

    let server = Server::new();

    server.start();
}
