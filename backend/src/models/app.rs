use redis::aio::ConnectionManager;

pub struct AppState {
    pub mongo: mongodb::Database,
    pub redis: ConnectionManager,
}
