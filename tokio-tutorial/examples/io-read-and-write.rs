use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    //// Fixed size buffer
    //let mut f = File::open("foo.txt").await?;
    //let mut buffer = [0; 10];
    //let n = f.read(&mut buffer[..]).await?;
    //println!("The bytes: {:?}", &buffer[..n]);

    //// Read 2 end
    //let mut f = File::open("foo.txt").await?;
    //let mut buffer = Vec::new();
    //f.read_to_end(&mut buffer).await?;
    //println!("{:?}", buffer);

    //// Write some
    //let mut f = File::create("foo-write.txt").await?;
    //let n = f
    //    .write(b"every saint got a past and every sinner got a future")
    //    .await?;
    //println!("Wrote the first {} bytes...", n);

    //// Write all
    //let mut f = File::create("foo-write.txt").await?;
    //f.write_all(b"every saint got a past and every sinner got a future")
    //    .await?;

    // Using tokio::io::copy
    let mut reader: &[u8] = b"hello";
    let mut file = File::create("foo-iocopy.txt").await?;
    io::copy(&mut reader, &mut file).await?;
    Ok(())
}
