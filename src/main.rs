mod reddit;

#[tokio::main]
async fn main() {
    let mut posts: Vec<reddit::RedditPost> = Vec::new();

    match reddit::fetch_reddit_posts().await {
        Ok(p) => {
            posts = p;
        }
        Err(e) => eprintln!("Error fetching posts: {:?}", e),
    };

    for post in &posts {
        println!("Post Text: {}\n\n\n\n", post.data.selftext)
    }
}
