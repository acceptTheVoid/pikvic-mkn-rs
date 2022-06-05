use rocket::serde::{Deserialize, Serialize};

use super::schema::items;

#[derive(Debug, Queryable, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub price: i32,
    pub description: String,
}

#[derive(Insertable)]
#[table_name = "items"]
pub struct NewItem<'a> {
    pub name: &'a str,
    pub price: i32,
    pub description: &'a str,
}
