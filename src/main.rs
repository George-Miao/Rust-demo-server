use std::net::SocketAddrV4;

use axum::{http::StatusCode, routing::get, Router};

#[tokio::main]
async fn main() {
    let port = std::env::var("PORT")
        .expect("PORT is not set")
        .parse()
        .expect("PORT is not valid");
    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .route("/health", get(|| async { StatusCode::NO_CONTENT }));

    axum::Server::bind(&SocketAddrV4::new("0.0.0.0".parse().unwrap(), port).into())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
