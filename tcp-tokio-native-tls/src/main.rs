use native_tls::Identity;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;

const PFX: &[u8] = include_bytes!("../../server.pfx");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let identity = Identity::from_pkcs12(PFX, "")?;

    let acceptor = native_tls::TlsAcceptor::new(identity)?;
    let acceptor = tokio_native_tls::TlsAcceptor::from(acceptor);

    let mut listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (client, _) = listener.accept().await?;
        let mut client = acceptor.clone().accept(client).await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            loop {
                let size = match client.read(&mut buf).await {
                    Ok(n) if n > 0 => n,
                    _ => break,
                };

                match client.write(&buf[0..size]).await {
                    Ok(n) if n == size => (),
                    _ => break,
                }
            }
        });
    }
}
