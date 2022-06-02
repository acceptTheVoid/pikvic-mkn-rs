#[macro_use]
pub extern crate rocket;
#[macro_use]
pub extern crate rocket_dyn_templates;

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

pub mod routes;
pub mod db;
