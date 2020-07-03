use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio_rustls::rustls::internal::pemfile::{certs, rsa_private_keys};
use tokio_rustls::rustls::{NoClientAuth, ServerConfig};
use tokio_rustls::TlsAcceptor;

const CRT: &[u8] = include_bytes!("../../server.crt");
const KEY: &[u8] = include_bytes!("../../server.key");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut keys = rsa_private_keys(&mut KEY).unwrap();
    let certs = certs(&mut CRT).unwrap();

    let mut config = ServerConfig::new(NoClientAuth::new());
    config.set_single_cert(certs, keys.remove(0))?;

    let acceptor = TlsAcceptor::from(Arc::new(config));

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
