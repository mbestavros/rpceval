use tokio::net::TcpListener;
use tokio::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            loop {
                let size = match socket.read(&mut buf).await {
                    Ok(n) if n > 0 => n,
                    _ => break,
                };

                match socket.write(&buf[0..size]).await {
                    Ok(n) if n == size => (),
                    _ => break,
                }
            }
        });
    }
}
