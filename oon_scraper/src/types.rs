use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ListingData {
    pub children: Vec<Post>,
    pub after: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Listing {
    pub data: ListingData,
}

#[derive(Deserialize, Debug)]
pub struct Post {
    pub data: PostData,
}

#[derive(Deserialize, Debug)]
pub struct PostData {
    pub title: String,
    pub name: String,
    pub permalink: String,
    pub subreddit: String,
}

impl PostData {
    pub fn full_permalink(&self) -> String {
        format!("https://reddit.com{}", self.permalink)
    }
}
