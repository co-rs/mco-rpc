use balance::LoadBalance;
use client::Client;
use mco::std::errors::Result;
use mco::std::sync::Mutex;

pub trait AddrFetcher {
    fn fetch_addr(&self) -> Vec<String>;
}

/// this is a connect manager.
/// Accepts a server addresses list，make a client list.
pub struct Manager {
    pub clients: LoadBalance<Client>,
    pub addr_fetcher: Box<dyn AddrFetcher>,
}

impl Manager {
    pub fn fetch(&mut self) -> Result<()> {
        let addrs = self.addr_fetcher.fetch_addr();
        for addr in addrs {
            if !self.clients.have(&addr) {
                let c = Client::dial(&addr)?;
                self.clients.put(c);
            }
        }
        return Ok(());
    }
}