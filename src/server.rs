use crate::handlers::health_check;
use axum::routing::get;
use axum::Router;
use std::net::SocketAddr;
use std::str::FromStr;
use tracing::info;

pub async fn init_server() {
    info!("Initialising post-guardian Service!");

    let api_routes = Router::new();

    let health_probe_routes: Router = Router::new().route("/healthz", get(health_check));
    let router = Router::new().merge(api_routes).merge(health_probe_routes);

    // Run server
    let addr = SocketAddr::from_str("0.0.0.0:7777").expect("Invalid server address");
    info!("server listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
