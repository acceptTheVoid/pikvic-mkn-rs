#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_dyn_templates;

use std::sync::Mutex;
use rocket::{serde::Deserialize, http::Status, response::Redirect};
use rocket_dyn_templates::Template;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
struct Item {
    name: String,
    price: usize,
}

struct State {
    items: Mutex<Vec<Item>>,
    cart: Mutex<Vec<Item>>,
}

#[get("/")]
fn index(state: &rocket::State<State>) -> Template {
    let items = &*state.items.lock().unwrap();
    let cart = &*state.cart.lock().unwrap();
    let ctx = context! { items, cart };

    Template::render("index", ctx)
}

#[get("/<id>")]
fn get_item(id: usize, state: &rocket::State<State>) -> Result<Template, Status> {
    let items = &*state.items.lock().unwrap();
    if items.len() <= id {
        return Err(Status::NotFound) 
    }

   Ok(Template::render("item", context!{ item: items[id].clone(), id }))
}

#[get("/cart/add?<id>")]
fn cart_add(id: Option<usize>, state: &rocket::State<State>) -> Redirect {
    let items = &*state.items.lock().unwrap();
    if let Some(id) = id {
        if id < items.len() {    
            let cart = &mut *state.cart.lock().unwrap();
            cart.push(items[id].clone());
        }
    }

    Redirect::to(uri!("/"))
}

#[get("/cart/remove?<id>")]
fn cart_remove(id: Option<usize>, state: &rocket::State<State>) -> Redirect {
    if let Some(id) = id {
        let cart = &mut *state.cart.lock().unwrap();
        if id < cart.len() {
            cart.remove(id);
        }
    }

    Redirect::to(uri!("/"))
}

#[launch]
fn rocket() -> _ {
    let state = State { 
        items: Mutex::new(vec![
            Item {name: String::from("asdfjasd"), price: 324},
            Item {name: String::from("asdsdsdad"), price: 3224},
            Item {name: String::from("asASddsaasd"), price: 3124},
            Item {name: String::from("4234232432asASddsaasd"), price: 3124}
        ]),
        cart: Mutex::new(vec![])
    };

    rocket::build()
        .mount("/", routes![index, get_item, cart_add, cart_remove])
        .manage(state)
        .attach(Template::fairing())
}
