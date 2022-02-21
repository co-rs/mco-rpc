use mco::std::errors::Result;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub trait ClientStub {
    fn call<Arg: Serialize, Resp: DeserializeOwned>(&self, method: &str, arg: Arg) -> Result<Resp>;
}

pub trait ServerStub {
    fn call<Arg: DeserializeOwned, Resp: Serialize>(&self, method: &str, arg: Arg) -> Result<Resp>;
}