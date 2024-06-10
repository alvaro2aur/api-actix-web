use actix_web::{web, App, HttpServer};

mod app;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at port 8081");
    HttpServer::new(|| {
        App::new()
        .service(web::scope("/summary").configure(app::routes::init))
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}
