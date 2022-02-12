use mini_redis::{client, Result};
use std::time::Duration;

async fn test_mini_redis() -> Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;

    client.set("hello", "world".into()).await?;
    let result = client.get("hello").await?;
    println!("Got value from the server; result=", result);
    //dbg!(result);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // TODO: error handling
    // tokio::join!(say_hello_world(), test_mini_redis());
    test_mini_redis().await?;
    Ok(())
}

// Rust's async operations are lazy
// The return value of an async fn is an anonymous type that implements the Future trait
// #[tokio::main] function is a macro. It transforms the async fn main() into synchronous

// TCP, UDP, Unix, sockets, timers, sync utilities, multiple scheduler types (joint!)

async fn say_world() {
    println!("world");
}

async fn say_hello_world() {
    let future = say_world();
    println!("hello");
    tokio::time::sleep(Duration::from_secs(2)).await;
    future.await;
}
