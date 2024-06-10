use actix_web::web;

pub mod hello;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(hello::detail).service(hello::summary);
}
