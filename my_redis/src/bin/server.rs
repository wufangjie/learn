use bytes::Bytes;
use mini_redis::Command::{self, Get, Set};
use mini_redis::{Connection, Frame};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let db = db.clone(); // move local db, next loop catch outer db
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            // NOTE: move, it's an async block
            process(socket, db).await;
        });
    }
}

async fn process(socket: TcpStream, db: Arc<Mutex<HashMap<String, Bytes>>>) {
    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        println!("GOT: {:?}", frame);
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                // dbg!(&db);
                // cmd.value().to_vec()); // Vec<u8> Bytes
                // Bytes ~~ Arc<Vec<u8>> + some added capabilities
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let db = db.lock().unwrap();
                // dbg!(&db);
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            _ => Frame::Error("Unknown operation!".to_string()),
        };
        // Frame type
        // dbg!(&response);
        // TODO: how can the client get the frame?
        connection.write_frame(&response).await.unwrap();
    }
}
