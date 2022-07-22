// use axum::{
//     Router,
//     service::get,
// };
// use tower_http::services::ServeDir;

// // Serves files inside the `public` directory at `GET /public/*`
// let serve_dir_service = ServeDir::new("public");

// let app = Router::new().nest("/public", get(serve_dir_service));