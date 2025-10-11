use mongodb::{Client, Database, bson::doc, options::ClientOptions};
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

    client.database("bike_shop")
}
