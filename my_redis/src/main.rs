use bytes::{Buf, Bytes, BytesMut};
use mini_redis::frame::Error::Incomplete;
use mini_redis::{Frame, Result};
use std::error::Error;
use std::io::Cursor;
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt, BufWriter};
use tokio::net::{TcpListener, TcpStream};
use utils::dbgt;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6142").await?;
    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut conn = Connection::new(socket);
            let frame = conn.read_frame().await.unwrap();
            dbg!(&frame);
            conn.write_frame(&Frame::Simple("Ok".to_string()))
                .await
                .unwrap();
        });
    }
}

#[derive(Debug)]
struct Connection {
    stream: BufWriter<TcpStream>,
    //buffer: Vec<u8>, //BytesMut,
    buffer: BytesMut,
    cursor: usize,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream: BufWriter::new(stream),
            buffer: BytesMut::with_capacity(4096), //Vec::with_capacity(4096),
            // vec![0; 4096], //,
            cursor: 0,
        }
    }

    pub async fn read_frame(&mut self) -> Result<Option<Frame>> {
        loop {
            if let Some(frame) = self.parse_frame()? {
                return Ok(Some(frame));
            }

            if self.buffer.len() == self.cursor {
                self.buffer.resize(self.cursor << 1, 0); // default value 0
            }

            //match self.stream.read(&mut self.buffer[self.cursor..]).await {
            match self.stream.read_buf(&mut self.buffer).await {
                Ok(0) => {
                    dbg!("0000");
                    return Ok(None);
                }
                Ok(n) => {
                    dbg!(&self);
                    self.cursor += n;
                }
                Err(_) => return Err("just test".into()),
            }
        }
    }

    fn parse_frame(&mut self) -> Result<Option<Frame>> {
        let mut buf = Cursor::new(&self.buffer[self.cursor..]); // NOTE: cursor
        match Frame::check(&mut buf) {
            Ok(_) => {
                let len = buf.position() as usize;
                buf.set_position(0);
                let frame = Frame::parse(&mut buf).unwrap();
                self.cursor += len;
                return Ok(Some(frame));
            }
            Err(Incomplete) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn write_frame(&mut self, frame: &Frame) -> Result<()> {
        match frame {
            Frame::Simple(val) => {
                self.stream.write_u8(b'+').await?;
                self.stream.write_all(val.as_bytes()).await?;
                self.stream.write_all(b"\r\n").await?;
            }
            Frame::Error(val) => {
                self.stream.write_u8(b'-').await?;
                self.stream.write_all(val.as_bytes()).await?;
                self.stream.write_all(b"\r\n").await?;
            }
            Frame::Integer(val) => {
                self.stream.write_u8(b':').await?;
                // self.stream.write_all(val.as_bytes()).await?;
                // self.stream.write_all(b"\r\n").await?;
                self.write_decimal(*val).await?;
                // TODO:
            }
            Frame::Null => {
                self.stream.write_all(b"$-1\r\n").await?;
            }
            Frame::Bulk(val) => {
                let len = val.len();

                self.stream.write_u8(b'$').await?;
                self.write_decimal(len as u64).await?;
                self.stream.write_all(val).await?;
                self.stream.write_all(b"\r\n").await?;
            }
            Frame::Array(_arr) => {
                self.stream.write_u8(b'*').await?;
                self.stream.write_all(b"\r\n").await?;
                return Err("Unimplemented!".into());
            }
        }
        self.stream.flush().await?; // Another alternative implement
        Ok(())
    }

    async fn write_decimal(&mut self, val: u64) -> io::Result<()> {
        use std::io::Write;

        // Convert the value to a string (actually ascii)
        let mut buf = [0u8; 12]; // MAX 12
        let mut buf = Cursor::new(&mut buf[..]);
        write!(&mut buf, "{}", val)?;

        let pos = buf.position() as usize;
        self.stream.write_all(&buf.get_ref()[..pos]).await?;
        self.stream.write_all(b"\r\n").await?;

        Ok(())
    }
}

// async fn read_foo_10() -> io::Result<()> {
//     let mut f = File::open("foo.txt").await?;
//     let mut buffer = [0; 10];
//     let n = f.read(&mut buffer[..]).await?;
//     println!("The bytes: {:?}", &buffer[..n]);
//     Ok(())
// }

// async fn read_foo_end() -> io::Result<()> {
//     let mut f = File::open("foo.txt").await?;
//     let mut buffer = vec![];
//     let n = f.read_to_end(&mut buffer).await?;
//     println!("The bytes: {:?}", &buffer);
//     Ok(())
// }

// async fn write_foo() -> io::Result<()> {
//     let mut f = File::create("foo.txt").await?; // even if it already existed
//     let n = f.write(b"some bytes").await?; // f.write_all
//     println!("Wrote {} bytes to foo.txt", n);
//     Ok(())
// }
