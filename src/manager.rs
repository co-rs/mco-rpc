use std::sync::Arc;
use std::time::Duration;
use mco::{co, err};
use mco::coroutine::sleep;
use balance::{LoadBalance, LoadBalanceType};
use client::Client;
use mco::std::errors::Result;
use mco::std::sync::Mutex;
use serde::de::DeserializeOwned;
use serde::Serialize;

/// to fetch remote service addr list
pub trait Fetcher: Sync + Send {
    ///fetch addrs
    fn fetch(&self) -> Vec<String>;
}

#[derive(Debug, Clone)]
pub struct ManagerConfig {
    pub balance: LoadBalanceType,
    pub interval: Duration,
}

/// this is a connect manager.
/// Accepts a server addresses list，make a client list.
pub struct Manager {
    pub config: ManagerConfig,
    pub clients: LoadBalance<Client>,
    pub fetcher: Box<dyn Fetcher>,
}

impl Manager {
    pub fn new<F>(cfg: ManagerConfig, f: F) -> Self where F: Fetcher + 'static {
        Self {
            config: cfg,
            clients: LoadBalance::new(),
            fetcher: Box::new(f),
        }
    }

    /// fetch addr list
    pub fn fetch(&self) -> Result<()> {
        let addrs = self.fetcher.fetch();
        for addr in addrs {
            if !self.clients.have(&addr) {
                let c = Client::dial(&addr)?;
                self.clients.put(c);
            }
        }
        return Ok(());
    }

    pub fn spawn_fetch(m: Arc<Manager>) {
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

    pub fn call<Arg, Resp>(&self, func: &str, arg: Arg) -> Result<Resp> where Arg: Serialize, Resp: DeserializeOwned {
        return match self.clients.do_balance(self.config.balance, "") {
            None => {
                Err(err!("no client to call!"))
            }
            Some(c) => {
                c.call(func, arg)
            }
        };
    }

    pub fn call_all<Arg, Resp>(&self, func: &str, arg: Arg, ip: &str) -> Result<Resp> where Arg: Serialize, Resp: DeserializeOwned {
        return match self.clients.do_balance(self.config.balance, ip) {
            None => {
                Err(err!("no client to call!"))
            }
            Some(c) => {
                c.call(func, arg)
            }
        };
    }
}