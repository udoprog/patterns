use anyhow::{bail, Result};
use bytes::{Buf, BytesMut};
use std::io::Cursor;
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> Result<()> {
    let blob = (0..(2u32 << 10))
        .map(|n| (n % 256) as u8)
        .collect::<Vec<_>>();

    let blob2 = blob.clone();

    let task = tokio::spawn(async move {
        let listener = TcpListener::bind("127.0.0.1:1111").await?;
        let mut buf = BytesMut::with_capacity(4096);

        loop {
            let (c, _) = listener.accept().await?;
            let received = receiver(c, blob2.len(), &mut buf).await?;
            assert_eq!(blob2, received);
        }

        Ok::<_, anyhow::Error>(())
    });

    loop {
        let client = TcpStream::connect("127.0.0.1:1111").await?;
        sender(client, &blob).await?;
    }
}

async fn sender(mut client: TcpStream, blob: &[u8]) -> Result<()> {
    use tokio::io::AsyncWriteExt as _;

    let mut cursor = Cursor::new(blob);

    while cursor.has_remaining() {
        client.write_buf(&mut (&mut cursor).take(4096)).await?;
        client.flush().await?;
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    }

    Ok(())
}

async fn receiver(mut client: TcpStream, n: usize, buf: &mut BytesMut) -> Result<Vec<u8>> {
    use bytes::BufMut as _;
    use tokio::io::AsyncReadExt as _;

    while buf.len() < n {
        let n = client.read_buf(buf).await?;

        if n == 0 {
            bail!("unexpected eof");
        }
    }

    let mut vec = vec![0u8; n];
    (&mut vec[..]).put_slice(&buf[..n]);
    buf.advance(n);
    Ok(vec)
}
