use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Duration;
use fast_log::sleep;
use mco::{co};
use mco_rpc::{BalanceManger, RegistryCenter, ManagerConfig};
use mco_rpc::server::Server;
use mco::std::errors::Result;
use mco_redis::client::Client;
use mco_redis::cmd;
use mco_redis::connector::RedisConnector;

pub struct RedisCenter {
    c: Client,
}

impl RedisCenter {
    pub fn new() -> Self {
        Self {
            c: RedisConnector::new("127.0.0.1:6379".to_string()).connect().expect("connect redis 127.0.0.1:6379"),
        }
    }
}

impl RegistryCenter for RedisCenter {
    fn pull(&self) -> HashMap<String, Vec<String>> {
        let mut m = HashMap::new();
        m.insert("test".to_string(), vec!["127.0.0.1:10000".to_string()]);
        m
    }

    fn push(&self, service: String, addr: String) -> Result<()> {
        if let Ok(v) = self.c.exec(cmd::Get(&service)) {
            let data = String::from_utf8(v.unwrap_or_default().to_vec()).unwrap_or_default();
            let mut addrs: Vec<String> = serde_json::from_str(&data).unwrap_or_default();
            if !addrs.contains(&addr) {
                addrs.push(addr.clone());
            }
            self.c.exec(cmd::Set(service.clone(), serde_json::to_string(&addrs).unwrap_or_default())).unwrap();
            return Ok(());
        }
        self.c.exec(cmd::Set(service, serde_json::to_string(&vec![addr]).unwrap_or_default())).unwrap();
        return Ok(());
    }
}

fn main() {
    let center = Arc::new(RedisCenter::new());
    let cener_clone = center.clone();
    co!(|| {
        spawn_server(cener_clone);
    });
    sleep(Duration::from_secs(2));
    let m = Arc::new(BalanceManger::new(ManagerConfig::default(), center));
    let m_clone = m.clone();
    co!(move ||{
       m_clone.spawn_pull();
    });
    sleep(Duration::from_secs(2));
    let r = m.call::<i32, i32>("test", "handle", 1);
    println!("{}", r.unwrap());
}

fn spawn_server(center: Arc<RedisCenter>) {
    center.push("test".to_string(), "127.0.0.1:10000".to_string());
    let mut s = Server::default();
    s.register_fn("handle", |arg: i32| -> Result<i32>{
        Ok(1)
    });
    s.serve("127.0.0.1:10000");
}