use rocket_test::{connect, models::Item, routes, schema::items, state};

#[macro_use]
extern crate rocket;
extern crate rocket_dyn_templates;

use rocket::tokio;
use rocket_dyn_templates::Template;

use diesel::prelude::*;

#[launch]
fn rocket() -> _ {
    let conn = connect();
    let items = items::table
        .load::<Item>(&conn)
        .expect("Failed to load items");

    println!("{:#?}", items);

    let state = state::State {
        conn: tokio::sync::Mutex::new(conn),
        items: std::sync::Mutex::new(items),
    };

    rocket::build()
        .mount("/", routes![routes::index, routes::get_item])
        .attach(Template::fairing())
        .manage(state)
}
