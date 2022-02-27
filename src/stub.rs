use mco::std::errors::Result;
use serde::de::DeserializeOwned;
use serde::Serialize;
use codec::Codecs;

/// The server address message is stored,
/// and the client request parameters are packaged into a network message,
/// which is then sent to the server remotely over the network
pub struct ClientStub {}

impl ClientStub {
    fn call<Arg: Serialize, Resp: DeserializeOwned>(&self, method: &str, arg: Arg, codec: &Codecs) -> Result<Resp> {
        todo!()
    }
}

/// Receives the message sent by the client, unpacks the me ssage, and invokes the local method.
pub struct ServerStub {}

impl ServerStub {
    fn call<Arg: DeserializeOwned, Resp: Serialize>(&self, method: &str, arg: Arg, codec: &Codecs) -> Result<Resp> {
        todo!()
    }
}


