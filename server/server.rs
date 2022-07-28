
use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, get_service},
    Router,
};
use std::{io, net::SocketAddr};
use tower_http::{services::ServeDir};

#[tokio::main]
async fn main() {
    let app: _ = Router::new()
        .route("/LR35902", get(|| async { "echo LR35902" }))
        .fallback(get_service(ServeDir::new("../")).handle_error(handle_error));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handle_error(_err: io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}