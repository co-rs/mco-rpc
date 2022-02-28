use std::io::Sink;
use std::net::SocketAddr;
use std::process::exit;
use std::time::Duration;
use fast_log::config::Config;
use fast_log::filter::ModuleFilter;
use mco::coroutine::{sleep, spawn};
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
    fast_log::init(Config::new()
        .console()
        .filter(ModuleFilter::new_exclude(vec!["mco_rpc::".to_string()])));
    spawn(|| {
        sleep(Duration::from_secs(1));
        let c = Client::dial("127.0.0.1:10000").unwrap();
        //c.codec = Codecs::JsonCodec(JsonCodec{});
        println!("dial success");
        let resp:i32 = c.call("handle",1).unwrap();
        println!("resp=>>>>>>>>>>>>>> :{}",resp);
        exit(0);
    });
    let mut s = Server::default();
    //s.codec = Codecs::JsonCodec(JsonCodec{});
    s.register("handle",H {});
    s.serve("0.0.0.0:10000");
    println!("Hello, world!");
}
