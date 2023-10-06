use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use star_wars_sdk::AllFilmsQuery;

#[tokio::main]
async fn main() {

    tracing_subscriber::fmt::init();



    let app = Router::new()
        .route("/", get(root))
        .route("/all-films", get(all_films))
        .route("/users", post(create_user));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

}

async fn root() -> String {
    format!("API v{}", 1)
}

async fn all_films(
) -> (StatusCode, Json<AllFilmsQuery>) {
    let response = star_wars_sdk::get_all_films().await;

    (StatusCode::OK, Json(response.data.unwrap()))
}

async fn create_user(
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    let user = User {
        id: 1337,
        username: payload.username,
    };

    (StatusCode::CREATED, Json(user))
}

#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}