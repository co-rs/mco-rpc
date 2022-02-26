use mco::net::TcpStream;
use codec::{BinCodec, Codec, Codecs};
use stub::ClientStub;

pub struct Client {
    codec: Codecs,
    stub: ClientStub,
    stream: TcpStream,
}

impl Client {
    pub fn dial() -> std::io::Result<Self> {
        let stream = TcpStream::connect(("0.0.0.0", 10000))?;
        Ok(Self {
            codec: Codecs::BinCodec(BinCodec {}),
            stub: ClientStub {},
            stream: stream,
        })
    }
}