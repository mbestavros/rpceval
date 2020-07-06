use std::sync::Arc;

use futures::sink::SinkExt;
use tokio::net::TcpListener;
use tokio::stream::StreamExt;
use tokio_rustls::rustls::internal::pemfile::{certs, rsa_private_keys};
use tokio_rustls::rustls::{NoClientAuth, ServerConfig};
use tokio_rustls::TlsAcceptor;
use tokio_util::codec::Framed;

const CRT: &[u8] = include_bytes!("../../server.crt");
const KEY: &[u8] = include_bytes!("../../server.key");

use serde_cbor::codec::Codec;
use serde_cbor::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut keys = rsa_private_keys(&mut KEY).unwrap();
    let certs = certs(&mut CRT).unwrap();

    let mut config = ServerConfig::new(NoClientAuth::new());
    config.set_single_cert(certs, keys.remove(0))?;

    let acceptor = TlsAcceptor::from(Arc::new(config));

    let mut listener = TcpListener::bind("127.0.0.1:8080").await?;

    while let Some(Ok(tcp)) = listener.next().await {
        let acceptor = acceptor.clone();

        tokio::spawn(async move {
            let mut cbor = match acceptor.accept(tcp).await {
                Err(e) => return Err(e),
                Ok(tls) => Framed::new(tls, Codec::<Value, Value>::default()),
            };

            while let Some(Ok(value)) = cbor.next().await {
                cbor.send(&value).await?;
            }

            Ok(())
        });
    }

    Ok(())
}
