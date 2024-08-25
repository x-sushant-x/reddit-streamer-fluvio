use std::{thread::sleep, time::Duration};

use fluvio::Fluvio;

mod reddit;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fluvio = Fluvio::connect().await?;
    let producer = fluvio.topic_producer("reddit-posts").await?;

    loop {
        let mut posts: Vec<reddit::RedditPost> = Vec::new();

        match reddit::fetch_reddit_posts().await {
            Ok(p) => {
                posts = p;
            }
            Err(e) => eprintln!("Error fetching posts: {:?}", e),
        };

        for post in &posts {
            let id = format!("{}", post.data.id);
            let text = format!("{}", post.data.selftext);
            producer.send(id, text).await?;
        }

        println!("Produce Completed");
        sleep(Duration::from_secs(30));
    }
}
