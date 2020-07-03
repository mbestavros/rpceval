This repository attempts to measure the cost in binary size of adopting
a particular framework for network communications. To reproduce my
results, do the following:

```
$ cargo +nightly build --release
...

$ cargo +nightly run --release --bin measure
    Finished release [optimized] target(s) in 0.08s
     Running `target/x86_64-unknown-linux-musl/release/measure`
                 tcp █████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ 325.89KB
           tcp-tokio ███████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ 458.05KB
    tcp-tokio-rustls ███████████████████████░░░░░░░░░░░░░░░░░░░░░░░░░░░ 1.35MB
tcp-tokio-native-tls ██████████████████████████████████████████████████ 2.84MB
    http-warp-rustls █████████████████████████████████████░░░░░░░░░░░░░ 2.11MB
  http-rocket-rustls ██████████████████████████████████████████████░░░░ 2.65MB
        jsonrpc-http ███████████████████████████████████████████████░░░ 2.72MB
         jsonrpc-tcp ██████████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ 821.99KB
      cborrpc-rustls ████████████████████████░░░░░░░░░░░░░░░░░░░░░░░░░░ 1.41MB
```

# Unresolved Questions

Several of these tests pull `lazy_static` into the dependency chain.
Enarx may need to grow features to support that.
