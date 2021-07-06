extern crate diesel_with_postgres;
extern crate dotenv;

use diesel_with_postgres::DieselDemo;
use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Error loading the database url");
    let diesel_demo = DieselDemo::new(database_url);
    diesel_demo.run();
}