use crate::models::Item;
use diesel::SqliteConnection;
use rocket::tokio;

pub struct State {
    pub conn: tokio::sync::Mutex<SqliteConnection>,
    pub items: std::sync::Mutex<Vec<Item>>,
}
