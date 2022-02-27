use std::io::{Read, Write};
use std::sync::atomic::{AtomicU64, Ordering};
use mco::net::TcpStream;
use mco::std::errors::Result;
use serde::de::DeserializeOwned;
use serde::{Serialize, Deserialize};
use codec::{Codec, Codecs};

#[derive(Serialize, Deserialize)]
pub struct PackHeader {
    pub method: String,
    pub tag: u64,
    pub length: usize,
}

/// and the client request parameters are packaged into a network message,
/// which is then sent to the server remotely over the network
pub struct ClientStub {
    pub tag: AtomicU64,
}

impl ClientStub {
    pub fn new() -> Self {
        Self {
            tag: AtomicU64::new(0)
        }
    }

    pub fn call<Arg: Serialize + 'static, Resp: DeserializeOwned>(&self, method: &str, arg: Arg, codec: &Codecs, stream: &mut TcpStream) -> Result<Resp> {
        let req = codec.encode(arg)?;
        let header = PackHeader { method: method.to_string(), tag: self.tag.load(Ordering::SeqCst) + 1, length: req.len() };
        let header_data = codec.encode(header)?;
        stream.write_all(&header_data);
        stream.write_all(&req);
        stream.flush()?;
        let mut buf_header = Vec::with_capacity(1024);
        loop {
            buf_header.clear();
            let read_len = stream.read(&mut buf_header)?;
            if read_len != 0 {
                let h: PackHeader = codec.decode(&buf_header)?;
                let mut buf = {
                    let mut buf = Vec::with_capacity(h.length);
                    for _ in 0..h.length {
                        buf.push(0);
                    }
                    buf
                };
                loop {
                    stream.read(&mut buf)?;
                    let resp: Resp = codec.decode(&buf)?;
                    return Ok(resp);
                }
            }
        }
    }
}

/// Receives the message sent by the client, unpacks the me ssage, and invokes the local method.
pub struct ServerStub {}

impl ServerStub {
    pub fn new() -> Self {
        Self {}
    }
    pub fn call<Arg: DeserializeOwned, Resp: Serialize>(&self, method: &str, arg: Arg, codec: &Codecs, stream: &mut TcpStream) -> Result<Resp> {
        todo!()
    }
}


