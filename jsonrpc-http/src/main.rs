use jsonrpc_http_server::jsonrpc_core::*;
use jsonrpc_http_server::*;

fn main() {
    let mut io = IoHandler::default();
    io.add_method("say_hello", |_| Ok(Value::String("hello".into())));

    let server = ServerBuilder::new(io)
        .cors(DomainsValidation::AllowOnly(vec![
            AccessControlAllowOrigin::Null,
        ]))
        .start_http(&"127.0.0.1:3030".parse().unwrap())
        .expect("Unable to start RPC server");

    server.wait()
}
