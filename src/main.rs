#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_dyn_templates;

use rocket::{fs::FileServer, serde::{Serialize, Deserialize}};
use rocket_dyn_templates::Template;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Post {
    user: &'static str,
    title: &'static str,
    content: &'static str,
    date: &'static str,
}

const POSTS: [Post; 3] = [
    Post {
        user: "Pete",
        title: "Warning!",
        content: "Lorem ipsum hello world!",
        date: "29-02-1988"
    },  Post {
        user: "Alex",
        title: "Caution!",
        content: "Text 2 is ipsum hello world!",
        date: "38-02-1988"
    }, Post {
        user: "Mary",
        title: "Hey!",
        content: "Go here!",
        date: "38-02-1988"
    }
];

#[get("/")]
fn index() -> Template {
    let ctx = context! { 
        posts: POSTS,
    };
    Template::render("template", &ctx)
}

#[get("/hello?<name>")]
fn hello(name: Option<&str>) -> String {
    let name = match name {
        Some(name) => name,
        None => "George",
    };

    format!("Hello, {name}!")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from("static/"))
        .mount("/", routes![index, hello])
        .attach(Template::fairing())
}
