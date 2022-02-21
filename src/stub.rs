use mco::std::errors::Result;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub trait ClientStub {
    fn call<Arg: Serialize, Resp: DeserializeOwned>(&self, method: &str, arg: Arg, resp: &mut Resp) -> Result<()>;
}

pub trait ServerStub {
    fn call<Arg: DeserializeOwned, Resp: Serialize>(&self, method: &str, arg: Arg, resp: &mut Resp) -> Result<()>;
}