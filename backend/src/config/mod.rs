use dotenvy::dotenv;
use env_logger::Env;

pub fn init() {
    dotenv().ok();
    env_logger::init_from_env(Env::default());
}
