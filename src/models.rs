use rocket::{serde::{Serialize, Deserialize}, form::FromForm};
use diesel::{Queryable, Insertable};
use crate::schema::*;

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

#[derive(Debug, Queryable, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, FromForm, Insertable, Clone)]
#[serde(crate = "rocket::serde")]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub password: String,
}
