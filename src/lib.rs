pub mod models;
pub mod routes;
pub mod schema;
pub mod state;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_dyn_templates;

use diesel::{sqlite::SqliteConnection, Connection};

use std::env;

pub fn connect() -> SqliteConnection {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connection to {database_url}!"))
}
