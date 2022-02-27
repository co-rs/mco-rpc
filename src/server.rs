use std::any::Any;
use mco::{co, err};
use mco::net::{TcpListener, TcpStream};
use codec::{BinCodec, Codec, Codecs};
use stub::ServerStub;
use std::io::Read;
use std::io::Write;
use std::net::ToSocketAddrs;
use mco::std::sync::SyncHashMap;
use serde::de::DeserializeOwned;
use serde::Serialize;
use mco::std::errors::Result;

pub struct Server {
    handles: SyncHashMap<String, Box<dyn Stub>>,
    codec: Codecs,
    stub: ServerStub,
}

impl Default for Server {
    fn default() -> Self {
        Self {
            handles: SyncHashMap::new(),
            codec: Codecs::BinCodec(BinCodec {}),
            stub: ServerStub {},
        }
    }
}


macro_rules! t {
    ($e:expr) => {
        match $e {
            Ok(val) => val,
            Err(err) => return println!("err = {:?}", err),
        }
    };
}

#[inline]
fn handle_client(mut stream: TcpStream) {
    let mut read = vec![0; 1024 * 16]; // alloc in heap!
    loop {
        let n = t!(stream.read(&mut read));
        if n > 0 {
            t!(stream.write_all(&read[0..n]));
        } else {
            break;
        }
    }
}

pub trait Stub {
    fn accept(&self, arg: &[u8], codec: &Codecs) -> Result<Vec<u8>>;
}

pub trait Handler {
    type Req: DeserializeOwned + 'static;
    type Resp: Serialize + 'static;
    fn handle(&self, req: Self::Req) -> Result<Self::Resp>;
}

impl<H: Handler> Stub for H {
    fn accept(&self, arg: &[u8], codec: &Codecs) -> Result<Vec<u8>> {
        //.or_else(|e| Result::Err(err!("{}",e)))?
        let req: H::Req = codec.decode(arg)?;
        let data = self.handle(req)?;
        Ok(codec.encode(data)?)
    }
}

// impl<Req, Resp, F> Handler for F where
//     Req: DeserializeOwned + 'static, Resp: Serialize + 'static,
//     F: Fn(Req) -> Resp,
//     F: Sync + Send
// {
//     type Req = Req;
//     type Resp = Resp;
//
//     fn handle(&self, req: Self::Req) -> Result<Self::Resp> {
//         self(req)
//     }
// }

impl Server {
    pub fn register<H: 'static>(&mut self, name: &str, handle: H) where H: Handler {
        self.handles.insert(name.to_owned(), Box::new(handle));
    }

    pub fn serve<A>(&self, addr: A) where A: ToSocketAddrs {
        let listener = TcpListener::bind(addr).unwrap();
        println!(
            "Starting tcp echo server on {:?}",
            listener.local_addr().unwrap(),
        );
        for stream in listener.incoming() {
            match stream {
                Ok(s) => {
                    co!(move || handle_client(s));
                }
                Err(e) => println!("err = {:?}", e),
            }
        }
    }
}