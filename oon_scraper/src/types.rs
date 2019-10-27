use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ListingData {
    pub children: Vec<Post>,
    pub after: String,
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
}
