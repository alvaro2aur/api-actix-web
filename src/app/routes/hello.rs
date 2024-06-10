use actix_web::{get, web, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Info {
    pub idta: u64
}

#[get("/{idta}")]
pub async fn summary(path: web::Path<Info>) -> impl Responder {
    let idta = path.into_inner().idta;
    format!("Hello, world! IDTA: {}", idta)
}

#[get("/detail")]
pub async fn detail() -> impl Responder {
    "Hello, world detail!"
}
