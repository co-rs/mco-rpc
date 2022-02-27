use std::io::{Read, Write};
use std::ops::Index;
use std::sync::atomic::{AtomicU64, Ordering};
use mco::err;
use mco::net::TcpStream;
use mco::std::errors::Result;
use mco::std::map::SyncHashMap;
use serde::de::DeserializeOwned;
use serde::{Serialize, Deserialize};
use codec::{Codec, Codecs};
use server::Stub;

#[derive(Serialize, Deserialize)]
pub struct PackHeader {
    pub m: String,//method
    pub t: u64,//tag
    pub l: usize,//len
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
        let header = PackHeader { m: method.to_string(), t: self.tag.load(Ordering::SeqCst) + 1, l: req.len() };
        let header_data = codec.encode(header)?;
        println!("header:{}", String::from_utf8(header_data.clone()).unwrap_or_default());
        println!("body:{}", String::from_utf8(req.clone()).unwrap_or_default());
        stream.write_all(&header_data);
        stream.write_all("\n".as_bytes());//write eof
        println!("header_len:{}", header_data.len());
        stream.write_all(&req);
        stream.write_all("\n".as_bytes());//write eof
        stream.flush()?;
        let mut buf_header = {
            let mut buf = Vec::with_capacity(4096);
            for _ in 0..4096 {
                buf.push(0);
            }
            buf
        };
        loop {
            reset(&mut buf_header);
            let read_len = stream.read_to_end(&mut buf_header)?;
            if read_len != 0 {
                println!("header-read len:{}", read_len);
                let buf_header = &buf_header[0..read_len];
                println!("header-resp:{}", String::from_utf8(buf_header.to_vec()).unwrap_or_default());
                if let Ok(h) = codec.decode::<PackHeader>(&buf_header) {
                    let mut buf = {
                        let mut buf = Vec::with_capacity(h.l);
                        for _ in 0..h.l {
                            buf.push(0);
                        }
                        buf
                    };
                    let read_len = stream.read(&mut buf)?;
                    println!("body-resp:{}", String::from_utf8(buf.clone()).unwrap_or_default());
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
    pub fn call(&self, stubs: &SyncHashMap<String, Box<dyn Stub>>, codec: &Codecs, stream: &mut TcpStream) -> Result<()> {
        let mut buf_header = {
            let mut buf = Vec::with_capacity(4096);
            for _ in 0..4096 {
                buf.push(0);
            }
            buf
        };
        loop {
            reset(&mut buf_header);
            let read_len = stream.read(&mut buf_header)?;
            if read_len != 0 {
                let buf_header = &buf_header[0..read_len];
                println!("header-read-server len:{}", read_len);
                println!("header-read-server:{}", String::from_utf8(buf_header.to_vec()).unwrap_or_default());
                if let Ok(h) = codec.decode::<PackHeader>(&buf_header) {
                    let stub = stubs.get(&h.m);
                    if stub.is_none() {
                        return Err(err!("method {} not find!",h.m));
                    }
                    let stub = stub.unwrap();
                    let mut buf = {
                        let mut buf = Vec::with_capacity(h.l);
                        for _ in 0..h.l {
                            buf.push(0);
                        }
                        buf
                    };
                    let read_len = stream.read_to_end(&mut buf)?;

                    println!("body-read-server:{}", String::from_utf8(buf.to_vec()).unwrap_or_default());

                    let r = stub.accept(&buf, codec)?;
                    for x in r {
                        buf.push(x);
                    }
                    stream.write_all(&buf)?;
                    stream.write_all(&[0]);//write eof
                    stream.flush()?;
                    return Ok(());
                }
            }
        }
    }
}

fn reset(buf: &mut Vec<u8>) {
    for x in buf {
        *x = 0;
    }
}