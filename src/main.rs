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
pub mod routes;

use routes::*;
use rocket_dyn_templates::Template;

#[launch]
fn rocket() -> _ {
    let state = state::DBConn::new();

    rocket::build()
        .mount("/", routes![index, category, login_get, login_post, logout])
        .attach(Template::fairing())
        .manage(state)
}
