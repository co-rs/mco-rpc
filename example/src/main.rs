use std::io::Sink;
use std::net::SocketAddr;
use std::thread::{sleep, spawn};
use std::time::Duration;
use serde::de::DeserializeOwned;
use serde::Serialize;
use mco_rpc::client::Client;
use mco_rpc::codec::{Codecs, JsonCodec};
use mco_rpc::server::{Handler, Server, Stub};

pub struct H {}

impl Handler for H {
    type Req = i32;
    type Resp = i32;

    fn handle(&self, req: Self::Req) -> mco::std::errors::Result<Self::Resp> {
        Ok(req)
    }
}


fn main() {
    spawn(|| {
        sleep(Duration::from_secs(1));
        let mut c = Client::dial(SocketAddr::from(([127, 0, 0, 1], 10000))).unwrap();
        c.codec = Codecs::JsonCodec(JsonCodec{});
        println!("dial success");
        let resp:i32 = c.call("handle",1).unwrap();
        println!("resp:{}",resp)
    });
    let mut s = Server::default();
    s.codec = Codecs::JsonCodec(JsonCodec{});
    s.register("handle",H {});
    s.serve(("0.0.0.0", 10000));
    println!("Hello, world!");
}
