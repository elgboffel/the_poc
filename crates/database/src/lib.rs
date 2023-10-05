use std::env;
use std::string::ToString;
use dotenvy::dotenv;
use sea_orm::{Database, DatabaseConnection, EntityTrait};
use entity::post;
use entity::prelude::Post;

fn connection_string_builder() -> String {
    dotenv().ok();

    let database_client: String = env::var("DATABASE_CLIENT")
        .expect("Failed to get variable DATABASE_CLIENT from .env")
        .to_string();
    let database_host: String = env::var("DATABASE_HOST")
        .expect("Failed to get variable DATABASE_HOST from .env")
        .to_string();
    let database_port: String = env::var("DATABASE_PORT")
        .expect("Failed to get variable DATABASE_PORT from .env")
        .to_string();
    let database_name: String = env::var("DATABASE_NAME")
        .expect("Failed to get variable DATABASE_NAME from .env")
        .to_string();
    let database_password: String = env::var("DATABASE_PASSWORD")
        .expect("Failed to get variable DATABASE_PASSWORD from .env")
        .to_string();
    let database_username: String = env::var("DATABASE_USERNAME").
        expect("Failed to get variable DATABASE_USERNAME from .env")
        .to_string();

    let database_url: String = format!("{}://{}:{}@{}:{}/{}", database_client, database_username, database_password, database_host, database_port, database_name);

    database_url
}

pub type DbConnection = DatabaseConnection;
pub async fn connect_db() -> DbConnection {
    let connection_string = connection_string_builder();

    Database::connect(&connection_string)
        .await
        .expect(format!("Failed to connect to databse {}", connection_string).as_str())
}

pub async fn get_all_posts(db: &DatabaseConnection) -> Vec<post::Model> {
    Post::find().all(db).await.expect("Failed to get all posts")
}
