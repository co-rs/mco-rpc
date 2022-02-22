use codec::{Codec, Codecs};
use stub::ClientStub;

pub struct Client {
    codec: Codecs,
    stub: ClientStub,
}