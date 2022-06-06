#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_dyn_templates;

pub mod database;
pub mod models;
pub mod schema;
pub mod state;

use rocket::State;
use rocket_dyn_templates::Template;
use state::DBConn;
use database::*;

#[get("/")]
async fn index(conn: &State<DBConn>) -> Template {
    let conn = &*conn.conn.lock().await;
    let categories = database::get_categories(conn);

    Template::render("index", context!{ categories })
}

#[get("/<cat_id>")]
async fn category(cat_id: i32, conn: &State<DBConn>) -> Option<Template> {
    let conn = &*conn.conn.lock().await;
    if let Some(category) = get_category(cat_id, conn) {
        let items = get_items(cat_id, conn);
        return Some(Template::render("category", context!{ category, items }));
    }

    None
}

#[launch]
fn rocket() -> _ {
    let state = state::DBConn::new();

    rocket::build()
        .mount("/", routes![index, category])
        .attach(Template::fairing())
        .manage(state)
}
