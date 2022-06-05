use crate::{models::Item, schema::items, *};
use diesel::prelude::*;
use rocket::State;
use rocket_dyn_templates::Template;

#[get("/")]
pub async fn index(state: &State<state::State>) -> Template {
    let items = state.items.lock().unwrap();
    let ctx = context! { items: &*items };

    Template::render("index", ctx)
}

#[get("/items/<name>")]
pub async fn get_item(name: &str, state: &State<state::State>) -> Template {
    use schema::items::dsl;

    let conn = state.conn.lock().await;
    let item = items::table
        .filter(dsl::name.eq(name))
        .first::<Item>(&*conn)
        .unwrap();

    let ctx = context! { item };

    Template::render("item", ctx)
}
