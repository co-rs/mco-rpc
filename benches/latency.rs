#![feature(test)]
#[macro_use]
extern crate mco_rpc;

#[cfg(test)]
extern crate test;
extern crate mco;

use std::net::SocketAddr;
use std::time::Duration;
#[cfg(test)]
use test::Bencher;
use mco::coroutine::{sleep, spawn};
use mco_rpc::client::Client;
use mco_rpc::codec::{Codecs, JsonCodec};
use mco_rpc::server::{Handler, Server};


pub struct H {}

impl Handler for H {
    type Req = i32;
    type Resp = i32;

    fn handle(&self, req: Self::Req) -> mco::std::errors::Result<Self::Resp> {
        Ok(req)
    }
}



#[cfg(test)]
#[bench]
fn latency(bencher: &mut Bencher) {
    spawn(move ||{
        let mut s = Server::default();
        //s.codec = Codecs::JsonCodec(JsonCodec{});
        s.register("handle",H {});
        s.serve(("127.0.0.1", 10000));
        println!("rpc served");
    });
    sleep(Duration::from_secs(1));
    let mut c = Client::dial("127.0.0.1:10000").unwrap();
    //c.codec = Codecs::JsonCodec(JsonCodec{});
    println!("dial success");
    let resp:i32 = c.call("handle",1).unwrap();
    println!("resp=>>>>>>>>>>>>>> :{}",resp);

    bencher.iter(||{
        let resp:i32 = c.call("handle",1).unwrap();
    });
}
