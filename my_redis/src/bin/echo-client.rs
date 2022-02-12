use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpSocket, TcpStream};

// #[tokio::main]
// async fn main() -> io::Result<()> {
//     let stream = TcpStream::connect("127.0.0.1:6142").await?;
//     let (mut rd, mut wr) = io::split(stream);
//     let h = tokio::spawn(async move {
// 	println!("Send: hello tcp stream!");
// 	wr.write_all(b"hello tcp stream!").await.unwrap();
// 	// wr.write_all(&[]).await.unwrap(); // not work
// 	wr.shutdown().await.unwrap(); // NOTE: important
//     });

//     h.await.unwrap();

//     let mut buf = [0u8; 128];
//     loop {
//     	let n = rd.read(&mut buf).await?;
//     	if n == 0 {
//     	    break;
//     	}
//     	println!("Got {:?}", &buf[..n]);
// 	dbg!(String::from_utf8_lossy(&buf[..n]));
//     }

//     // let mut buf = vec![];
//     // rd.read_to_end(&mut buf).await?;
//     // dbg!(String::from_utf8_lossy(&buf)); // FIXME: how to drop the stream
//     Ok(())
// }

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:6142").await?;
    //let (mut rd, mut wr) = io::split(stream);

    stream
        .write_all(b"*3\r\n$3\r\nset\r\n$4\r\ndemo\r\n$6\r\n123456\r\n+OK\r\n")
        .await
        .unwrap();
    // wr.write_all(&[]).await.unwrap(); // not work
    stream.shutdown().await.unwrap(); // NOTE: important

    let mut buf = [0u8; 128];
    loop {
        let n = stream.read(&mut buf).await?;
        if n == 0 {
            break;
        }
        println!("Got {:?}", &buf[..n]);
        dbg!(String::from_utf8_lossy(&buf[..n]));
    }

    // let mut buf = vec![];
    // rd.read_to_end(&mut buf).await?;
    // dbg!(String::from_utf8_lossy(&buf)); // FIXME: how to drop the stream
    Ok(())
}
