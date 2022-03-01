use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use mco::{co, err};
use mco::coroutine::sleep;
use balance::{LoadBalance, LoadBalanceType};
use client::Client;
use mco::std::errors::Result;
use mco::std::sync::{Mutex, SyncHashMap};
use serde::de::DeserializeOwned;
use serde::Serialize;

/// to fetch remote service addr list
pub trait Fetcher: Sync + Send {
    ///fetch addrs
    fn fetch(&self) -> HashMap<String, Vec<String>>;
}

#[derive(Debug, Clone)]
pub struct ManagerConfig {
    pub balance: LoadBalanceType,
    pub interval: Duration,
}

/// this is a connect manager.
/// Accepts a server addresses list，make a client list.
pub struct BalanceManger {
    pub config: ManagerConfig,
    pub clients: SyncHashMap<String, LoadBalance<Client>>,
    pub fetcher: Box<dyn Fetcher>,
}

impl BalanceManger {
    pub fn new<F>(cfg: ManagerConfig, f: F) -> Self where F: Fetcher + 'static {
        Self {
            config: cfg,
            clients: SyncHashMap::new(),
            fetcher: Box::new(f),
        }
    }

    /// fetch addr list
    pub fn fetch(&self) -> Result<()> {
        let addrs = self.fetcher.fetch();
        for (s, addrs) in addrs {
            let balance = self.clients.get(&s);
            if let Some(clients) = balance {
                for addr in &addrs {
                    if !clients.have(addr) {
                        let c = Client::dial(addr)?;
                        clients.put(c);
                    }
                }
            } else {
                let mut clients = LoadBalance::new();
                for x in addrs {
                    let c = Client::dial(&x)?;
                    clients.put(c);
                }
                self.clients.insert(s, clients);
            }
        }
        return Ok(());
    }

    pub fn spawn_fetch(m: Arc<BalanceManger>) {
        co!(move ||{
            loop{
               let r = m.fetch();
               if r.is_err(){
                    log::error!("service fetch fail:{}",r.err().unwrap());
               }
               sleep(m.config.interval);
            }
        });
    }

    pub fn call<Arg, Resp>(&self, service: &str, func: &str, arg: Arg) -> Result<Resp> where Arg: Serialize, Resp: DeserializeOwned {
        return match self.clients.get(service)
            .ok_or(err!("no service :{} find!",service))?
            .do_balance(self.config.balance, "") {
            None => {
                Err(err!("no client to call!"))
            }
            Some(c) => {
                c.call(func, arg)
            }
        };
    }

    pub fn call_all<Arg, Resp>(&self, service: &str, func: &str, arg: Arg, ip: &str) -> Result<Resp> where Arg: Serialize, Resp: DeserializeOwned {
        return match self.clients
            .get(service).ok_or(err!("no service :{} find!",service))?
            .do_balance(self.config.balance, ip) {
            None => {
                Err(err!("no client to call!"))
            }
            Some(c) => {
                c.call(func, arg)
            }
        };
    }
}

#[cfg(test)]
mod test{
    #[test]
    fn test_fetch(){

    }
}