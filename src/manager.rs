use std::sync::Arc;
use mco::err;
use balance::{LoadBalance, LoadBalanceType};
use client::Client;
use mco::std::errors::Result;
use mco::std::sync::Mutex;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub trait Fetcher {
    ///fetch addrs
    fn fetch(&self) -> Vec<String>;
}

/// this is a connect manager.
/// Accepts a server addresses list，make a client list.
pub struct Manager {
    pub balance: LoadBalanceType,
    pub clients: LoadBalance<Client>,
    pub fetcher: Box<dyn Fetcher>,
}

impl Manager {
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

    pub fn call<Arg, Resp>(&self, func: &str, arg: Arg) -> Result<Resp> where Arg: Serialize, Resp: DeserializeOwned {
        return match self.clients.do_balance(self.balance, "") {
            None => {
                Err(err!("no client to call!"))
            }
            Some(c) => {
                c.call(func, arg)
            }
        };
    }

    pub fn call_all<Arg, Resp>(&self, func: &str, arg: Arg, ip: &str) -> Result<Resp> where Arg: Serialize, Resp: DeserializeOwned {
        return match self.clients.do_balance(self.balance, ip) {
            None => {
                Err(err!("no client to call!"))
            }
            Some(c) => {
                c.call(func, arg)
            }
        };
    }
}