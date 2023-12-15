use std::env;

mod client;
mod connection;
mod server;
mod websocket;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <port>", args[0]);
        std::process::exit(1);
    }

    let port = &args[1];
    server::run_server(port);
}
