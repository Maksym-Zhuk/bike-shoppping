use actix_web::web;

pub mod order;
pub mod product;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(product::init());
    cfg.service(order::init());
}
