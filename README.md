# mco-rpc

mco-rpc

* based T-L-V.for example:  ```[Tag][Length][Value]```
* support json/bincode
* support load balance

## how to use?

* client

```rust
use mco_rpc::client::Client;
let c = Client::dial("127.0.0.1:10000").unwrap();
let resp:i32 = c.call("handle", 1).unwrap();
println!("resp=>>>>>>>>>>>>>> :{}", resp);
```

* server

```rust
use mco_rpc::server::{Handler, Server, Stub};

pub struct H {}

fn handle(req: i32) -> mco::std::errors::Result<i32> {
    Ok(req)
}
let mut s = Server::default ();
s.register_fn("handle", handle);
s.register_fn("handle_fn2", |arg:i32| -> Result<i32>{
Ok(1)
});
s.serve("0.0.0.0:10000");
```