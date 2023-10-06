use axum::{routing::{get, post}, http::StatusCode, Json, Router, ServiceExt};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use axum::extract::State;
use star_wars_sdk::AllFilmsQuery;
use dotenvy::dotenv;
// use database::DbConnection;
use entity::post;


// #[derive(Clone)]
// struct AppState {
//     db: DbConnection
// }


#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt::init();

    // let state = AppState {
    //     db: database::connect_db().await
    // };

    let app = Router::new()
        .route("/", get(root))
        .route("/all-films", get(all_films))
        // .route("/all-posts", get(all_posts))
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

// #[axum_macros::debug_handler]
// async fn all_posts(
//     State(state): State<AppState>,
// ) -> (StatusCode, Json<Vec<post::Model>>) {
//
//     let response = database::get_all_posts(&state.db).await;
//
//     (StatusCode::OK, Json(response))
// }

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