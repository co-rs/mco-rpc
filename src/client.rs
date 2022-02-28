use std::cell::RefCell;
use std::net::{SocketAddr, ToSocketAddrs};
use mco::net::TcpStream;
use codec::{BinCodec, Codec, Codecs};
use stub::ClientStub;
use mco::std::errors::Result;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub struct Client {
    pub addr: String,
    pub codec: Codecs,
    pub stub: ClientStub,
    pub stream: RefCell<TcpStream>,
}

impl Client {
    pub fn dial(addr: &str) -> std::io::Result<Self> {
        let address = addr.to_string();
        let stream = TcpStream::connect(addr)?;
        Ok(Self {
            addr: address,
            codec: Codecs::BinCodec(BinCodec {}),
            stub: ClientStub::new(),
            stream: RefCell::new(stream),
        })
    }

    pub fn call<Arg, Resp>(&self, func: &str, arg: Arg) -> Result<Resp> where Arg: Serialize, Resp: DeserializeOwned {
        let resp: Resp = self.stub.call(func, arg, &self.codec, &mut *self.stream.borrow_mut())?;
        return Ok(resp);
    }
}