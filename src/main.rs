#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_dyn_templates;

use std::{fs, sync::Mutex};

use rocket::serde::{Deserialize, Serialize, json::Json};
use rocket_dyn_templates::Template;

#[derive(Debug, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
struct News {
    id: usize,
    title: String,
    content: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct NewNews<'a> {
    title: &'a str,
    content: &'a str,
}

struct State {
    news: Mutex<Vec<News>>,
}

#[get("/")]
fn index(news: &rocket::State<State>) -> Template {
    let news = &*news.news.lock().unwrap();
    Template::render("index", context!{ news })
}

#[get("/news/<id>")]
fn get_content(id: usize, news: &rocket::State<State>) -> Json<News> {
    let news = &*news.news.lock().unwrap();
    Json(news[id - 1].clone())
}

#[post("/news", data = "<data>")]
fn post_news(data: Json<NewNews<'_>>, news: &rocket::State<State>) -> Json<News> {
    let news = &mut *news.news.lock().unwrap();
    let data = News {
        id: news.len() + 1,
        title: data.title.to_string(),
        content: data.content.to_string(),
    };

    news.push(data.clone());
    Json(data)
}

#[launch]
fn rocket() -> _ {
    let news = get_news().unwrap();
    
    println!("{:#?}", news);

    let news = Mutex::new(news);
    let state = State { news };

    rocket::build()
        .mount("/", routes![index, get_content, post_news])
        .manage(state)
        .attach(Template::fairing())
}

fn get_news() -> Result<Vec<News>, std::io::Error> {
    let contents = fs::read_to_string("news.txt")?;
    let mut lines = contents.lines();

    let mut news = vec![];
    if let Some(n) = lines.next() {
        let n = n.parse::<usize>().unwrap();
        for _ in 0..n {
            let id = lines.next().unwrap().parse::<usize>().unwrap();
            let title = lines.next().unwrap().to_string();
            let content = lines.next().unwrap().to_string();
            let item = News { id, title, content };

            news.push(item);
        }
    }

    Ok(news)
}
