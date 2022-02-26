use mco::co;
use mco::net::{TcpListener, TcpStream};
use codec::{BinCodec, Codec, Codecs};
use stub::ServerStub;
use std::io::Read;
use std::io::Write;

pub struct Server {
    codec: Codecs,
    stub: ServerStub,
}

impl Default for Server {
    fn default() -> Self {
        Self {
            codec: Codecs::BinCodec(BinCodec {}),
            stub: ServerStub {},
        }
    }
}


macro_rules! t {
    ($e:expr) => {
        match $e {
            Ok(val) => val,
            Err(err) => return println!("err = {:?}", err),
        }
    };
}

#[inline]
fn handle_client(mut stream: TcpStream) {
    // t!(stream.set_read_timeout(Some(Duration::from_secs(10))));
    // t!(stream.set_write_timeout(Some(Duration::from_secs(10))));
    let mut read = vec![0; 1024 * 16]; // alloc in heap!
    loop {
        let n = t!(stream.read(&mut read));
        if n > 0 {
            t!(stream.write_all(&read[0..n]));
        } else {
            break;
        }
    }
}

impl Server {
    pub fn serve(&self) {
        let listener = TcpListener::bind(("0.0.0.0", 10000)).unwrap();
        println!(
            "Starting tcp echo server on {:?}",
            listener.local_addr().unwrap(),
        );
        for stream in listener.incoming() {
            match stream {
                Ok(s) => {
                    co!(move || handle_client(s));
                }
                Err(e) => println!("err = {:?}", e),
            }
        }
    }
}