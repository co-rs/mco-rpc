use mco::net::TcpStream;
use mco::std::errors::Result;
use serde::de::DeserializeOwned;
use serde::Serialize;
use codec::{Codec, Codecs};

/// and the client request parameters are packaged into a network message,
/// which is then sent to the server remotely over the network
pub struct ClientStub {}

impl ClientStub {
    pub fn call<Arg: Serialize + 'static, Resp: DeserializeOwned>(&self, method: &str, arg: Arg, codec: &Codecs, stream: &mut TcpStream) -> Result<Resp> {
        let req = codec.encode(arg)?;

        todo!()
    }
}

/// Receives the message sent by the client, unpacks the me ssage, and invokes the local method.
pub struct ServerStub {}

impl ServerStub {
    pub fn call<Arg: DeserializeOwned, Resp: Serialize>(&self, method: &str, arg: Arg, codec: &Codecs) -> Result<Resp> {
        todo!()
    }
}


