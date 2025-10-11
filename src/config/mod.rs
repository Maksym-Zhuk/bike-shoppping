use dotenvy::dotenv;
use std::env;

pub fn init() {
    dotenv().ok();
    env_logger::init();
}
