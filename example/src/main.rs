use std::process::exit;
use std::time::Duration;
use fast_log::config::Config;
use fast_log::filter::ModuleFilter;
use mco::co;
use mco::coroutine::{sleep};
use mco_rpc::client::Client;
use mco_rpc::server::{Server};
use mco::std::errors::Result;

fn handle(req: i32) -> Result<i32> {
    return Ok(req + 1);
}

fn main() {
    _ = fast_log::init(Config::new()
        .console()
        .filter(ModuleFilter::new_exclude(vec!["mco_rpc::".to_string()])));
    co!(|| {
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
    s.register_fn("handle", handle);
    s.register_fn("handle_fn2", |_arg:i32| -> Result<i32>{
        Ok(1)
    });
    s.serve("0.0.0.0:10000");
    println!("Hello, world!");
}
