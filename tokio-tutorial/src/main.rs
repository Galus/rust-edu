use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut socket = TcpStream::connect("127.0.0.1:1337").await?;
    tokio::spawn(async move {
        let (mut rx, mut tx) = socket.split();

        if io::copy(&mut rx, &mut tx).await.is_err() {
            eprintln!("failed to copy");
        }
    });

    //let mut buf = vec![0; 128];
    //loop {
    //    let n = rx.read(&mut buf).await?;
    //    if n == 0 {
    //        break;
    //    }
    //    println!("Got {:?}", &buf[..n]);
    //}

    Ok(())
}
