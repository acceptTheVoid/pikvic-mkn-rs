#[macro_use]
pub extern crate rocket;

pub use rocket::State;
pub use rocket::form::{Form, Lenient};
pub use rocket::fs::FileServer;
pub use rocket::response::Redirect;
pub use rocket::tokio::io::{AsyncReadExt, AsyncWriteExt};
pub use rocket::tokio::fs::File;
pub use rocket::serde::{Serialize, Deserialize};

pub use rocket::tokio::sync::Mutex;
pub use rocket_dyn_templates::Template;

pub use std::path::{Path, PathBuf};
pub use std::io;
pub use std::sync::atomic::{Ordering, AtomicUsize};

use rocket_test::{db::*, routes::*};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from("static/"))
        .mount("/", routes![index, add_post, append_post])
        .manage(Posts { posts: Mutex::new(Vec::new()) })
        .attach(Template::fairing())
}
