use mco::err;
use mco::std::errors::Error;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub trait Codec {
    fn encode<T: Serialize>(&self, arg: T) -> Result<Vec<u8>, Error>;
    fn decode<T: DeserializeOwned>(&self, arg: &[u8]) -> Result<T, Error>;
}


pub struct JsonCodec {}

impl Codec for JsonCodec {
    fn encode<T: Serialize>(&self, arg: T) -> Result<Vec<u8>, Error> {
        match serde_json::to_vec(&arg) {
            Ok(ok) => { Ok(ok) }
            Err(e) => { Err(err!("{}",e)) }
        }
    }

    fn decode<T: DeserializeOwned>(&self, arg: &[u8]) -> Result<T, Error> {
        match serde_json::from_slice(arg) {
            Ok(v) => {
                Ok(v)
            }
            Err(e) => {
                Err(err!("{}",e))
            }
        }
    }
}

pub struct BinCodec {}

impl Codec for BinCodec{
    fn encode<T: Serialize>(&self, arg: T) -> Result<Vec<u8>, Error> {
        match bincode::serialize(&arg) {
            Ok(ok) => { Ok(ok) }
            Err(e) => { Err(err!("{}",e)) }
        }
    }

    fn decode<T: DeserializeOwned>(&self, arg: &[u8]) -> Result<T, Error> {
        match bincode::deserialize(arg) {
            Ok(ok) => { Ok(ok) }
            Err(e) => { Err(err!("{}",e)) }
        }
    }
}