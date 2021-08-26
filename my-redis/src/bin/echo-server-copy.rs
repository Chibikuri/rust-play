use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6142").await.unwrap();

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            loop {
                match socket.read(&mut buf).await {
                    // remote closed
                    Ok(0) => return,
                    Ok(n) => {
                        // copy the data back to socket
                        if socket.write_all(&buf[..n]).await.is_err() {
                            // unexpected socket error
                            return;
                        }
                    }
                    Err(_) => {
                        return;
                    }
                }
            }
            // let (mut rd, mut wr) = socket.split();
            // if io::copy(&mut rd, &mut wr).await.is_err() {
            //     eprintln!("failed to copy");
            // }
        });
    }
}
