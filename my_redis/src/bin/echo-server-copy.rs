use tokio::fs::File;
use tokio::io::{self, AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::net::TcpListener;
use utils::dbgt;

// #[tokio::main]
// async fn main() -> io::Result<()> {
//     let listener = TcpListener::bind("127.0.0.1:6142").await?;
//     loop {
// 	let (mut socket, _) = listener.accept().await?;
// 	dbgt!(&socket);
// 	let (mut rd, mut wr) = io::split(socket);
// 	// let (mut rd, mut wr) = socket.split();
// 	let h1 = tokio::spawn(async move {
// 	    loop {
// 		let mut buf = [0u8; 128]; // vec![0u8; 128];
// 		let n = rd.read(&mut buf).await.unwrap();
// 		if n == 0 {
// 		    break;
// 		}
// 		println!("Got {:?}", &buf[..n]);
// 	    }
// 	});
// 	let h2 = tokio::spawn(async move {
// 	    println!("Send Ok");
// 	    wr.write(b"Ok").await.unwrap();
// 	});
// 	h1.await.unwrap();
// 	h2.await.unwrap();
//     }
// }

// #[tokio::main]
// async fn main() -> io::Result<()> {
//     let listener = TcpListener::bind("127.0.0.1:6142").await?;
//     loop {
// 	let (socket, _) = listener.accept().await?;
// 	// let (mut rd, mut wr) = socket.split();
// 	tokio::spawn(async move {
// 	    let (mut rd, mut wr) = io::split(socket);
// 	    wr.write(b"Got: ").await.unwrap(); // only failed when the other half dropped
// 	    if io::copy(&mut rd, &mut wr).await.is_err() {
// 		eprintln!("Fail to copy!");
// 	    }
// 	}).await;
//     }
// }

// #[tokio::main]
// async fn main() -> io::Result<()> {
//     let listener = TcpListener::bind("127.0.0.1:6142").await?;
//     loop {
// 	let (mut socket, _) = listener.accept().await?;
// 	tokio::spawn(async move {
// 	    //Connection
// 	    let mut buffer = vec![];
// 	    //io::copy(&mut socket, &mut buffer).await;
// 	    socket.read_to_end(&mut buffer).await;
// 	    //dbg!(("Got: {}", &buffer));
// 	    println!("Got: {}", String::from_utf8_lossy(&buffer));
// 	    //io::copy(&mut vec![b'O', b'k'], &mut socket).await;
// 	    socket.write_all(b"Ok").await;
// 	    socket.write_all(b"Hello").await;
// 	    socket.write_all(b"TcpStream").await;
// 	    socket.write_all(b"longerenough0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789").await;
// 	}).await;
//     }
// }

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6142").await?;
    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];
            socket.write(b"Got").await.unwrap();
            loop {
                match socket.read(&mut buf).await {
                    Ok(0) => {
                        dbg!("Received 0, been shutdown");
                        return;
                    }
                    Ok(n) => {
                        dbg!(String::from_utf8_lossy(&buf[..n]));
                        if socket.write(&buf[..n]).await.is_err() {
                            eprintln!("been dropped");
                            return;
                        }
                    }
                    Err(_) => return,
                }
            }
        })
        .await;
    }
}
