use mini_redis::{client, Result};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;
    client.set("tokio", "is hard".into()).await?;
    tokio::time::sleep(Duration::from_secs(6)).await;
    dbg!(client.get("hello").await?);
    client.set("rust", "hard to find a job".into()).await?;
    dbg!(client.get("rust").await?);
    tokio::time::sleep(Duration::from_secs(6)).await;
    dbg!(client.get("rust").await?);
    Ok(())
}
