use mco::mco_gen::Error;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub trait Codec {
    fn encode<T: Serialize>(&self, arg: T) -> Result<Vec<u8>, Error>;
    fn decode<T: DeserializeOwned>(&self, arg: &[u8]) -> Result<T, Error>;
}