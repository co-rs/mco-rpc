use std::io::Sink;
use std::net::SocketAddr;
use std::thread::{sleep, spawn};
use std::time::Duration;
use mco_rpc::client::Client;
use mco_rpc::server::Server;

fn main() {
    spawn(||{
       sleep(Duration::from_secs(1));
       let c = Client::dial(SocketAddr::from(([127, 0, 0, 1], 10000))).unwrap();
       println!("dial success");
    });
    let s =Server::default();
    s.serve();
    println!("Hello, world!");
}
