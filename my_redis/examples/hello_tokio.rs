use mini_redis::{client, Result};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;
    client.set("hello", "world".into()).await?;
    dbg!(client.get("hello").await?);
    dbg!(client.get("rust").await?);
    client.set("rust", "awesome".into()).await?;
    dbg!(client.get("rust").await?);
    Ok(())
}
