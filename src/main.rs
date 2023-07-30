#![allow(dead_code)]
#![warn(unused_imports)]

mod server;
mod http;
mod website_handler;

use server::Server;
use website_handler::WebsiteHandler;
use std::env;

fn main() {
    let default_path = format!("{}/public", env::var("CARGO_MAINFEST_DIR").unwrap());
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    let app = Server::new(String::from("127.0.0.1:8080"));
    app.run(WebsiteHandler::new(public_path));
}
