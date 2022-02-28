# mco-rpc

mco-rpc

* based on [mco](https://github.com/co-rs/mco), this is green-thread、coroutines crates
* based T-L-V.for example:  ```[Tag][Length][Value]```
* support json/bincode
* support load balance

## how to use?

```toml
mco="0.1"
mco-rpc = "0.1"
```

* client

```rust
use mco_rpc::client::Client;
let c = Client::dial("127.0.0.1:10000").unwrap();
let resp:i32 = c.call("handle", 1).unwrap();
println!("resp=>>>>>>>>>>>>>> :{}", resp);
```

* server

```rust
use mco_rpc::server::Server;
use mco::std::errors::Result;

fn handle(req: i32) -> Result<i32> {
    Ok(req)
}
let mut s = Server::default ();
s.register_fn("handle", handle);
s.register_fn("handle_fn2", |arg:i32| -> Result<i32>{
Ok(1)
});
s.serve("0.0.0.0:10000");
```