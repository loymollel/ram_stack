extern crate dotenv;
extern crate log;

use actix_web::{ web, App, HttpServer };
use sqlx::mysql::MySqlPoolOptions;

use dotenv::dotenv;
use std::{env};
use log::{info};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().expect("Failed to read .env file");
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("error"));

    info!("Starting up");

    let app_host = env::var("APP_HOST").expect("APP_HOST not found.");
    let app_port = env::var("APP_PORT").expect("APP_PORT not found.");
    let app_url = format!("{}:{}", &app_host, &app_port);
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not found in .env file.");

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Unable to connect to MySQL Database");

    info!("Listening on: {}", &app_url);

    // Start HTTP Server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
    })
        .bind(&app_url)?
        .run()
        .await
}
