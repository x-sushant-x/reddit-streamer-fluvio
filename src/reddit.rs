use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RedditPost {
    pub data: RedditPostData,
}

#[derive(Deserialize, Debug)]
pub struct RedditPostData {
    pub id: String,
    pub title: String,
    pub selftext: String,
}

pub async fn fetch_reddit_posts() -> Result<Vec<RedditPost>, Box<dyn std::error::Error>> {
    let api_endpoint = format!("https://www.reddit.com/r/golang/new.json");

    let client = Client::new();
    let resp = client
        .get(api_endpoint)
        .header("User-Agent", "rust-fluvio-bot")
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    let posts = resp["data"]["children"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .map(|p| {
            // println!("Post: {}\n", p["data"]["selftext"]);
            serde_json::from_value(p.clone()).unwrap()
        })
        .collect::<Vec<RedditPost>>();

    Ok(posts)
}
