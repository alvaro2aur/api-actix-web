use actix_web::{web, App, HttpServer};

mod app;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at port 8080");
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/summary")
                    .configure(app::routes::init)
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}