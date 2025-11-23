use std::env;

use diesel::{Connection, SqliteConnection};
use dotenvy::dotenv;

pub fn estabilish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL has to be set!");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
