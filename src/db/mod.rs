
use super::*;

const PATH: &str = "posts"; 

#[derive(Debug, Serialize, Deserialize, FromForm)]
#[serde(crate = "rocket::serde")]
pub struct Post {
    pub id: Option<usize>,
    pub user: String,
    pub title: String,
    pub date: Option<String>,
    pub content: String,
}

type P = Mutex<Vec<Post>>;

pub struct Posts {
    pub posts: P,
}

pub async fn get_posts(state_posts: &State<Posts>) -> io::Result<()> {
    let path = Path::new(PATH);
    let mut posts = Vec::with_capacity(path.read_dir().iter().len());

    for f in path.read_dir().unwrap() {
        let mut post = String::new();
        if let Ok(f) = f {
            File::open(f.path()).await?.read_to_string(&mut post).await?;            
        }

        let post = serde_json::from_str(&post).unwrap();
        posts.push(post);
    }

    if !posts.is_empty() {
        let mut lock = state_posts.posts.lock().await;
        *lock = posts;
    }

    Ok(())
}

pub async fn create_post(post: &Post) -> io::Result<()> {
    let path = PathBuf::from(format!("{PATH}/post{}.txt", post.id.unwrap()));
    let mut file = File::create(path).await?;
    file.write_all_buf(&mut serde_json::to_string(&post).unwrap().as_bytes()).await?;

    Ok(())
}

pub fn check_empty(post: &Post) -> bool {
    post.user.trim().is_empty() || post.title.trim().is_empty() || post.content.trim().is_empty()
}
