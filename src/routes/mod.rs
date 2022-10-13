use axum::{Router, routing::{get, post}};

pub mod hello;
pub mod leaderboard;

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(hello::hello_world))
        //.route("/leaderboard", post(leaderboard::top))
}