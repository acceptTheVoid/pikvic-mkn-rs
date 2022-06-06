use rocket::serde::{Serialize, Deserialize};
use diesel::Queryable;

#[derive(Debug, Queryable, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Category {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Queryable, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub price: i32,
    pub cat_id: i32,
}
