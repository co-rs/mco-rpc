use std::any::Any;
use mco::{co, err};
use mco::net::{TcpListener, TcpStream};
use codec::{BinCodec, Codec, Codecs};
use stub::ServerStub;
use std::io::Read;
use std::io::Write;
use std::net::ToSocketAddrs;
use std::sync::Arc;
use mco::std::sync::SyncHashMap;
use serde::de::DeserializeOwned;
use serde::Serialize;
use mco::std::errors::Result;

pub struct Server {
    pub handles: SyncHashMap<String, Box<dyn Stub>>,
    pub codec: Codecs,
    pub stub: ServerStub,
}

impl Default for Server {
    fn default() -> Self {
        Self {
            handles: SyncHashMap::new(),
            codec: Codecs::BinCodec(BinCodec {}),
            stub: ServerStub::new(),
        }
    }
}

impl Server {
    #[inline]
    pub fn call(&self, stream: TcpStream) {
        self.stub.call(&self.handles, &self.codec, stream);
    }
}

pub trait Stub {
    fn accept(&self, arg: &[u8], codec: &Codecs) -> Result<Vec<u8>>;
}

pub trait Handler:Stub {
    type Req: DeserializeOwned;
    type Resp: Serialize;
    fn accept(&self, arg: &[u8], codec: &Codecs) -> Result<Vec<u8>> {
        //.or_else(|e| Result::Err(err!("{}",e)))?
        let req: Self::Req = codec.decode(arg)?;
        let data = self.handle(req)?;
        Ok(codec.encode(data)?)
    }
    fn handle(&self, req: Self::Req) -> Result<Self::Resp>;
}

impl <H:Handler>Stub for H{
    fn accept(&self, arg: &[u8], codec: &Codecs) -> Result<Vec<u8>> {
        <H as Handler>::accept(self,arg,codec)
    }
}

// impl<F> Stub for F where F: Fn(i32)->Result<i32>, F: Sync + Send {
//     fn accept(&self, arg: &[u8], codec: &Codecs) -> Result<Vec<u8>> {
//         let req: i32 = codec.decode(arg)?;
//         let data = self(req)?;
//         Ok(codec.encode(data)?)
//     }
// }

impl Server {
    pub fn register<H: 'static>(&mut self, name: &str, handle: H) where H: Stub {
        self.handles.insert(name.to_owned(), Box::new(handle));
    }

    pub fn serve<A>(self, addr: A) where A: ToSocketAddrs {
        let listener = TcpListener::bind(addr).unwrap();
        println!(
            "Starting tcp echo server on {:?}",
            listener.local_addr().unwrap(),
        );
        let server = Arc::new(self);
        for stream in listener.incoming() {
            match stream {
                Ok(s) => {
                    let server = server.clone();
                    co!(move || server.call(s));
                }
                Err(e) => println!("err = {:?}", e),
            }
        }
    }
}