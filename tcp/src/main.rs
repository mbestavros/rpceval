use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            thread::spawn(move || {
                let mut buf = [0; 1024];

                while let Ok(size) = stream.read(&mut buf) {
                    match stream.write(&buf[0..size]) {
                        Ok(s) if s == size => continue,
                        _ => break,
                    }
                }
            });
        }
    }
}
