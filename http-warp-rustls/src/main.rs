use warp::Filter;

const CRT: &[u8] = include_bytes!("../../server.crt");
const KEY: &[u8] = include_bytes!("../../server.key");

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

    warp::serve(hello)
        .tls()
        .key(KEY)
        .cert(CRT)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
