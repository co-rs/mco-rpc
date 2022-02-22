use codec::{Codec, Codecs};
use stub::ServerStub;

pub struct Server {
    codec: Codecs,
    stub: ServerStub,
}