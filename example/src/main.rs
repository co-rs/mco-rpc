use std::io::Sink;
use mco_rpc::server::Server;

fn main() {
    let s =Server::default();
    s.serve();
    println!("Hello, world!");
}
