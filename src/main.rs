use std::env;

mod client;
mod connection;
mod server;
mod websocket;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <server|client> <port>", args[0]);
        std::process::exit(1);
    }

    let mode = &args[1];
    let port = &args[2];
    if mode == "client" {
        client::run_client(port);
    } else if mode == "server" {
        server::run_server(port);
    } else {
        eprintln!("Invalid mode: {}", mode);
        std::process::exit(1);
    }
}
