use rocket::tokio::sync::Mutex;
use diesel::prelude::*;
use std::env;

pub struct DBConn {
    pub conn: Mutex<SqliteConnection>,
}

impl DBConn {
    pub fn new() -> Self {
        dotenv::dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let conn = SqliteConnection::establish(&db_url)
            .expect(&format!("Error connecting to {db_url}!"));
        let conn = Mutex::new(conn);

        Self { conn }
    }
}
