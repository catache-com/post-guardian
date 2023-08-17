use crate::server::init_server;

mod db;
mod handlers;
mod models;
mod server;

#[tokio::main]
async fn main() {
    init_server().await;
}
