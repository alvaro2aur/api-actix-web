use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;
use dotenv::dotenv;

pub fn establish_connection() -> PgConnection {
    // dotenv().ok();  // Cargar variables de entorno del archivo .env
    println!("TERM_PROGRAM: {}", env::var("TERM_PROGRAM").unwrap());
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    //PgConnection::establish(&database_url)
    //    .expect(&format!("Error connecting to {}", database_url))
}
