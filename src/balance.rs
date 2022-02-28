use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use rand::Rng;
use client::Client;

pub struct LoadBalance {
    pub index: AtomicUsize,
    pub rpc_clients: Vec<Arc<Client>>,
    pub rpc_clients_map: HashMap<String, Arc<Client>>,
}

pub enum LoadBalanceType {
    LoadBalanceTypeRound,
    LoadBalanceTypeRandom,
    LoadBalanceTypeHASH,
}


impl LoadBalance {
    pub fn new() -> Self {
        Self {
            index: AtomicUsize::new(0),
            rpc_clients: vec![],
            rpc_clients_map: HashMap::new(),
        }
    }

    pub fn put(&mut self, arg: Client) {
        self.rpc_clients_map.insert(arg.addr.clone(), Arc::new(arg));
        self.rpc_clients.clear();
        for (_, v) in &self.rpc_clients_map {
            self.rpc_clients.push(v.clone());
        }
    }

    pub fn remove(&mut self, address: &str) {
        self.rpc_clients_map.remove(address);
        self.rpc_clients.clear();
        for (_, v) in &self.rpc_clients_map {
            self.rpc_clients.push(v.clone());
        }
    }
    pub fn clear(&mut self) {
        self.rpc_clients_map.clear();
        self.rpc_clients.clear();
    }

    pub fn do_balance(&self, b: LoadBalanceType, client_ip: &str) -> Option<Arc<Client>> {
        match b {
            LoadBalanceType::LoadBalanceTypeRound => {
                self.round_pick_client()
            }
            LoadBalanceType::LoadBalanceTypeRandom => {
                self.random_pick_client()
            }
            LoadBalanceType::LoadBalanceTypeHASH => {
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
        use rand::{thread_rng, Rng};
        let mut rng = thread_rng();
        let rand_index: usize = rng.gen_range(0..length);
        if rand_index < length {
            return Some(self.rpc_clients[rand_index].clone());
        }
        return None;
    }

    fn round_pick_client(&self) -> Option<Arc<Client>> {
        let idx = self.index.load(Ordering::SeqCst);
        let length = self.rpc_clients.len();
        if length == 0 {
            return None;
        }
        let return_obj = self.rpc_clients[idx].clone();
        if (idx + 1) > length {
            self.index.store(0, Ordering::SeqCst)
        } else {
            self.index.store(idx + 1, Ordering::SeqCst);
        }
        return Some(return_obj);
    }
}