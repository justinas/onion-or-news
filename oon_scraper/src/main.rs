mod types;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let page : types::Listing = reqwest::get("https://reddit.com/r/nottheonion.json")
        .await?
        .json()
        .await?;
    for post in page.data.children {
        println!("{:?}", post.data.title);
    }
    Ok(())
}
