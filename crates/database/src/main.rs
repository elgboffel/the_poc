use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;
use entity::post;

#[async_std::main]
async fn main() {
    let db = database::connect_db().await;

    let post = post::ActiveModel {
        title: Set(String::from("Amazing title 1")),
        text: Set(String::from("Lorem ipsum dolor sit amet.")),
        ..Default::default()
    };

    post.insert(&db).await;
}