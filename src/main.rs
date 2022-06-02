#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_dyn_templates;

use rand::Rng;
use rocket::fs::FileServer;
use rocket_dyn_templates::Template;

const NAMES: [&str; 4] = [
    "Alex",
    "Pete",
    "Arkadiy",
    "Kate",
];

static mut COUNTER: usize = 0;

#[get("/")]
fn index() -> Template {
    let mut rng = rand::thread_rng();
    let n = rng.gen_range(0..NAMES.len());
    let c = rng.gen_range(0..3);

    unsafe { COUNTER += 1 }
    let msg = format!("Hello, {}", NAMES[n]);
    let color = ["red", "blue", "black"][c];
    
    let ctx = context! { 
        msg: msg, 
        color: color, 
        counter: unsafe { COUNTER }, 
    };
    Template::render("template", &ctx)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from("static/"))
        .mount("/", routes![index])
        .attach(Template::fairing())
}
