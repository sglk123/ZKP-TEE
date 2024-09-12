mod enclave_main;

use tokio::io::{AsyncWriteExt, AsyncReadExt};
use tokio::net::UnixStream;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let cid = 3;
    let port = 5005;
    let addr = format!("/dev/vsock/{}.{}", cid, port);

    let mut stream = UnixStream::connect(addr).await?;

    let request = b"Hello, Enclave!";
    stream.write_all(request).await?;
    println!("Sent: {}", String::from_utf8_lossy(request));

    let mut buffer = [0u8; 1024];
    let n = stream.read(&mut buffer).await?;
    println!("Received: {}", String::from_utf8_lossy(&buffer[..n]));

    Ok(())
}
