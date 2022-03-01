use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use rand::Rng;
use client::Client;

#[derive(Debug)]
pub struct LoadBalance {
    pub index: AtomicUsize,
    pub rpc_clients: Vec<Arc<Client>>,
}

pub enum LoadBalanceType {
    Round,
    Random,
    Hash,
}


impl LoadBalance {
    pub fn new() -> Self {
        Self {
            index: AtomicUsize::new(0),
            rpc_clients: vec![],
        }
    }

    pub fn put(&mut self, arg: Client) {
        let mut arg = Some(Arc::new(arg));
        let addr = &arg.as_deref().unwrap().addr;
        for x in &mut self.rpc_clients {
            if x.addr.eq(addr) {
                *x = arg.take().unwrap();
                break;
            }
        }
        if let Some(arg) = arg {
            self.rpc_clients.push(arg);
        }
    }

    pub fn remove(&mut self, address: &str) {
        let mut idx = 0;
        let mut need_remove = None;
        for x in &self.rpc_clients {
            if x.addr.eq(address) {
                need_remove = Some(idx);
            }
            idx += 1;
        }
        if let Some(rm) = need_remove {
            self.rpc_clients.remove(rm);
        }
    }

    pub fn clear(&mut self) {
        self.rpc_clients.clear();
    }

    pub fn do_balance(&self, b: LoadBalanceType, client_ip: &str) -> Option<Arc<Client>> {
        match b {
            LoadBalanceType::Round => {
                self.round_pick_client()
            }
            LoadBalanceType::Random => {
                self.random_pick_client()
            }
            LoadBalanceType::Hash => {
                self.hash_pick_client(client_ip)
            }
        }
    }

    fn hash_pick_client(&self, client_ip: &str) -> Option<Arc<Client>> {
        let length = self.rpc_clients.len() as i64;
        if length == 0 {
            return None;
        }
        let def_key: String;
        if client_ip.is_empty() {
            def_key = format!("{}", rand::random::<i32>());
        } else {
            def_key = client_ip.to_string();
        }
        let hash = {
            let mut value = 0i64;
            let mut i = 0;
            for x in def_key.as_bytes() {
                i += 1;
                value += (*x as i64) * i;
            }
            value
        };
        return Some(self.rpc_clients[(hash % length) as usize].clone());
    }

    fn random_pick_client(&self) -> Option<Arc<Client>> {
        let length = self.rpc_clients.len();
        if length == 0 {
            return None;
        }
        use rand::{thread_rng, Rng};
        let mut rng = thread_rng();
        let rand_index: usize = rng.gen_range(0..length);
        if rand_index < length {
            return Some(self.rpc_clients[rand_index].clone());
        }
        return None;
    }

    fn round_pick_client(&self) -> Option<Arc<Client>> {
        let length = self.rpc_clients.len();
        if length == 0 {
            return None;
        }
        let idx = self.index.load(Ordering::SeqCst);
        if (idx + 1) >= length {
            self.index.store(0, Ordering::SeqCst)
        } else {
            self.index.store(idx + 1, Ordering::SeqCst);
        }
        let return_obj = self.rpc_clients[idx].clone();
        return Some(return_obj);
    }
}