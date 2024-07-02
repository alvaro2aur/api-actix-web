use std::env;

#[derive(Debug)]
pub struct Config {
    pub database_url: String,
}

impl Config {
    pub fn from_env(schema: String) -> Self {
        let user = env::var("POSTGRES_USER").expect("POSTGRES_USER must be set");
        let password = env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set");
        let server = env::var("POSTGRES_SERVER").expect("POSTGRES_SERVER must be set");
        let database = env::var("POSTGRES_DATABASE").expect("POSTGRES_DATABASE must be set");

        let database_url = format!("postgres://{}:{}@{}/{}?options=-c search_path={}", user, password, server, database, schema);
    
        Config { database_url }
    }
}