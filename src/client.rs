use std::cell::RefCell;
use std::net::{SocketAddr, ToSocketAddrs};
use mco::net::TcpStream;
use codec::{BinCodec, Codec, Codecs};
use stub::ClientStub;
use mco::std::errors::Result;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub struct Client {
    pub codec: Codecs,
    pub stub: ClientStub,
    pub stream: RefCell<TcpStream>,
}

impl Client {
    pub fn dial<A: ToSocketAddrs>(addr: A) -> std::io::Result<Self> {
        let stream = TcpStream::connect(addr)?;
        Ok(Self {
            codec: Codecs::BinCodec(BinCodec {}),
            stub: ClientStub::new(),
            stream: RefCell::new(stream),
        })
    }

    pub fn call<Arg, Resp>(&self, func: &str, arg: Arg) -> Result<Resp> where Arg: Serialize + 'static, Resp: DeserializeOwned {
        let resp: Resp = self.stub.call(func, arg, &self.codec, &mut *self.stream.borrow_mut())?;
        return Ok(resp);
    }
}