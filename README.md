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

impl Handler for H {
    type Req = i32;
    type Resp = i32;

    fn handle(&self, req: Self::Req) -> mco::std::errors::Result<Self::Resp> {
        Ok(req)
    }
}
let mut s = Server::default ();
s.register("handle", H {});
s.serve("0.0.0.0:10000");
```