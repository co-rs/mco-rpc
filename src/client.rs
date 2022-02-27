use std::net::{SocketAddr, ToSocketAddrs};
use mco::net::TcpStream;
use codec::{BinCodec, Codec, Codecs};
use stub::ClientStub;

pub struct Client {
    codec: Codecs,
    stub: ClientStub,
    stream: TcpStream,
}

impl Client {
    pub fn dial<A: ToSocketAddrs>(addr: A) -> std::io::Result<Self> {
        let stream = TcpStream::connect(addr)?;
        Ok(Self {
            codec: Codecs::BinCodec(BinCodec {}),
            stub: ClientStub {},
            stream: stream,
        })
    }
}