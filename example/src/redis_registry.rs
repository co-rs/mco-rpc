use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use fast_log::sleep;
use mco::{co, hash_map};
use mco_rpc::{BalanceManger, RegistryCenter, ManagerConfig};
use mco_rpc::server::Server;
use mco::std::errors::Result;

pub struct RedisFetcher {}

impl RegistryCenter for RedisFetcher {
    fn pull(&self) -> HashMap<String, Vec<String>> {
        let mut m = HashMap::new();
        m.insert("test".to_string(), vec!["127.0.0.1:10000".to_string()]);
        m
    }

    fn push(&self) -> Result<()> {
        todo!()
    }
}

fn main() {
    co!(|| {
        spawn_server();
    });
    let m = Arc::new(BalanceManger::new(ManagerConfig::default(), RedisFetcher {}));
    let m_clone = m.clone();
    co!(move ||{
       m_clone.spawn_pull();
    });
    sleep(Duration::from_secs(2));
    let r = m.call::<i32, i32>("test", "handle", 1);
    println!("{}", r.unwrap());
}

fn spawn_server() {
    let mut s = Server::default();
    s.register_fn("handle", |arg: i32| -> Result<i32>{
        Ok(1)
    });
    s.serve("127.0.0.1:10000");
}