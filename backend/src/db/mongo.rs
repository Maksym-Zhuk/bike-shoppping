use mongodb::{
    bson::doc,
    options::{ClientOptions, IndexOptions},
    Client, Database, IndexModel,
};
use std::env;

pub async fn init_db() -> Database {
    let db_url: String = env::var("DATABASE_URL").unwrap().parse().unwrap();
    let mut client_options = ClientOptions::parse(&db_url)
        .await
        .expect("Failed to parse MongoDB URI");

    client_options.app_name = Some("BikeShopApp".to_string());

    let client: Client =
        Client::with_options(client_options).expect("Failed to create MongoDB client");

    let _ = client
        .database("admin")
        .run_command(doc! {"ping": 1})
        .await
        .expect("Failed to ping MongoDB");

    println!("✅ Connecting to MongoDB is successful!");

    let db = client.database("bike_shop");

    ensure_indexes(&db).await;

    db
}

pub async fn ensure_indexes(db: &Database) {
    let users = db.collection::<mongodb::bson::Document>("users");
    let index_options = IndexOptions::builder().unique(true).build();
    let model = IndexModel::builder()
        .keys(doc! { "email": 1 })
        .options(index_options)
        .build();
    users.create_index(model).await.unwrap();
}
