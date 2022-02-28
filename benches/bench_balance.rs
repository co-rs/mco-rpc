#![feature(test)]
extern crate test;
extern crate mco_rpc;
extern crate mco;

use std::mem::MaybeUninit;
use std::thread::sleep;
use std::time::Duration;
use mco::co;
use mco_rpc::balance::{LoadBalance, LoadBalanceType};
use mco_rpc::client::Client;

#[bench]
fn bench_balance(b: &mut test::Bencher) {
    let mut load =LoadBalance::new();
    co!(||{
        let mut s = mco_rpc::server::Server::default();
        s.serve(("127.0.0.1", 3000));
    });
    co!(||{
        let mut s = mco_rpc::server::Server::default();
        s.serve(("127.0.0.1", 3001));
    });
    co!(||{
        let mut s = mco_rpc::server::Server::default();
        s.serve(("127.0.0.1", 3002));
    });
    co!(||{
        let mut s = mco_rpc::server::Server::default();
        s.serve(("127.0.0.1", 3003));
    });
    sleep(Duration::from_secs(2));
    load.put(Client::dial("127.0.0.1:3000").unwrap());
    load.put(Client::dial("127.0.0.1:3001").unwrap());
    load.put(Client::dial("127.0.0.1:3002").unwrap());
    load.put(Client::dial("127.0.0.1:3003").unwrap());
    b.iter(|| {
        load.do_balance(LoadBalanceType::LoadBalanceTypeRound,"");
    });
}