extern crate dotenv;

use actix_web::{ middleware, web, App, HttpServer };
use sqlx::mysql::MySqlPoolOptions;

use dotenv::dotenv;
use std::{env};

#[actix_web::main]
async fn main() -> Result<(), sqlx::Error> {

    dotenv().expect("Failed to read .env file");
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not found in .env file.");

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Unable to connect to MySQL Database");

    let email = "admin@email.com";
    let row: (i64,) = sqlx::query_as("SELECT * FROM users WHERE email = ?")
        .bind(email)
        .fetch_one(&pool)
        .await?;

    assert_eq!(row.0, 0);

    Ok(())
}
