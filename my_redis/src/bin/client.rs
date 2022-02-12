use bytes::Bytes;
use mini_redis::{client, Frame, Result};
// use std::sync::Arc; //, Mutex}; // tokio has no Arc
use tokio::sync::{mpsc, oneshot}; // Mutex

pub type Error = Box<dyn std::error::Error + Send + Sync>;
//pub type Responder = oneshot::Sender<Frame>;
pub type Responder<T> = oneshot::Sender<Result<T>>;

#[tokio::main]
async fn main() -> Result<()> {
    let (tx, mut rx) = mpsc::channel(32);
    let mut client = client::connect("127.0.0.1:6379").await?;

    let tx2 = tx.clone();
    let h1 = tokio::spawn(async move {
        let (tx, mut rx) = oneshot::channel();
        tx2.send(Command::Get {
            key: "hello".to_string(),
            resp: tx,
        })
        .await?;
        dbg!(rx.await?); //try_recv()?);
        Ok::<(), Error>(())
        // https://rust-lang.github.io/async-book/07_workarounds/02_err_in_async_blocks.html
        // .unwrap();
    });
    let tx2 = tx.clone();
    let h2 = tokio::spawn(async move {
        let (tx, mut rx) = oneshot::channel();
        tx2.send(Command::Set {
            key: "foo".to_string(),
            val: "bar".into(),
            resp: tx,
        })
        .await?;
        dbg!(rx.await);
        Ok::<(), Error>(())
    });
    let tx2 = tx.clone();
    let h3 = tokio::spawn(async move {
        let (tx, mut rx) = oneshot::channel();
        tx2.send(Command::Set {
            key: "hello".to_string(),
            val: "world".into(),
            resp: tx,
        })
        .await?;
        dbg!(rx.await);
        Ok::<(), Error>(())
    });

    let h0 = tokio::spawn(async move {
        while let Some(cmd) = rx.recv().await {
            match cmd {
                Command::Get { key, resp } => {
                    resp.send(client.get(&key).await);
                    //Ok::<(), Error>(())
                }
                Command::Set { key, val, resp } => {
                    resp.send(client.set(&key, val).await);
                }
            }
        }
    });

    drop(tx); // NOTE: drop tx, or rx.recv().await will not get `None`

    h3.await.unwrap(); // TODO: remove warning
    h2.await?; //.unwrap();
    h1.await?; //.unwrap();
    h0.await?;

    Ok(())
}

#[derive(Debug)]
enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Bytes,
        resp: Responder<()>,
    },
}
