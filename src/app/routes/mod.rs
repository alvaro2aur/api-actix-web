use actix_web::web;

pub mod summary;
pub mod models;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(summary::get_summary);
}
