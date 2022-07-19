// use axum::{
//     routing::{get, post},
//     Json, Router,
// };
// use std::net::SocketAddr;

// #[tokio::main]
// async fn main() {
//     let app = Router::new()
//         .route("/", get(root))

//     let addr = SocketAddr::from(([127, 0, 0, 1], 8888));
//     tracing::debug!("listening on {}", addr);
//     axum::Server::bind(&addr)
//         .serve(app.into_make_service())
//         .await
//         .unwrap();
// }

// // basic handler that responds with a static string
// async fn root() -> &'static str {
//     "Hello, World!"
// }
