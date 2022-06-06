use rocket::State;
use rocket::form::Form;
use rocket::http::{CookieJar, Cookie};
use rocket::response::Redirect;
use rocket_dyn_templates::Template;
use crate::models::NewUser;
use crate::state::DBConn;
use crate::database::{*, self};

#[get("/")]
pub async fn index(conn: &State<DBConn>, session: &CookieJar<'_>) -> Template {
    let conn = &*conn.conn.lock().await;
    let categories = database::get_categories(conn);

    println!("{:#?}", session.get_private("username"));
    let username = session.get_private("username")
        .map(|c| c.value().to_string());

    Template::render("index", context!{ categories, username })
}

#[get("/<cat_id>")]
pub async fn category(cat_id: i32, conn: &State<DBConn>, session: &CookieJar<'_>) -> Option<Template> {
    let conn = &*conn.conn.lock().await;

    if let Some(category) = get_category(cat_id, conn) {
        let username = session.get_private("username")
            .map(|c| c.value().to_string());
        let items = get_items(cat_id, conn);

        return Some(Template::render("category", context!{ username, category, items }));
    }
        
    None
}

#[get("/login")]
pub async fn login_get(session: &CookieJar<'_>) -> Result<Redirect, Template> {
    match session.get_private("username") {
        Some(_) => Ok(Redirect::to(uri!("/"))),
        None => Err(Template::render("login", context!{ }))
    }
}

#[post("/login", data = "<user>")]
pub async fn login_post(
    user: Form<NewUser>,
    conn: &State<DBConn>, 
    session: &CookieJar<'_>,
) -> Redirect {
    println!("{:#?}", user);
    println!("{:#?}", session.get_private("username"));
    let conn = &*conn.conn.lock().await;
    let users = get_users(&user.username, conn);
    println!("{:?}", users);
    if users.is_empty() {
        let user = (*user).clone();
        insert_user(&user, conn);
        let NewUser { username, .. } = user;
        session.add_private(Cookie::new("username", username));
        println!("{:#?}", session.get_private("username"));
        println!("User should be added....");
    } else {
        if users[0].password == user.password {
            println!("Users are matching");
            match session.get_private("username") {
                Some(mut c) => c.set_value(&user.username),
                None => {
                    let NewUser { username, .. } = (*user).clone();
                    session.add_private(Cookie::new("username", username));
                }
            }
        }
    }

    Redirect::to(uri!("/"))
}

#[get("/logout")]
pub fn logout(session: &CookieJar<'_>) -> Redirect {
    session.remove_private(Cookie::named("username"));
    Redirect::to("/")
}
