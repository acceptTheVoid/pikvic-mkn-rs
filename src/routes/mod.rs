use super::*;
use db::*;

#[get("/")]
pub async fn index(state: &State<Posts>) -> Template {
    get_posts(&state).await.unwrap();

    let posts = state.posts.lock().await;
    if !posts.is_empty() {
        Template::render("index", context! { posts: &*posts })
    } else {
        Template::render("no_posts", context! { })
    }
}

#[get("/add_post")]
pub async fn add_post() -> Template {
    Template::render("add_post", context! { })
}

#[post("/add_post", data = "<post>")]
pub async fn append_post(post: Form<Lenient<Post>>, posts: &State<Posts>) -> Redirect {
    if check_empty(&post) {
        return Redirect::to(uri!("/"));
    }

    let mut posts = posts.posts.lock().await;
    let id = Some(posts.len() + 1);
    let user = post.user.clone();
    let title = post.title.clone();
    let date = Some(chrono::Utc::now().format("%Y-%m-%d").to_string());
    let content = post.content.clone();

    let post = Post { id, user, title, date, content };
    create_post(&post).await.unwrap();
    posts.push(post);

    Redirect::to(uri!("/"))
}
