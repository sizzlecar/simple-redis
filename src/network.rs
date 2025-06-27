use crate::{Resp, RespDecoder};
use crate::{RespEncoder, RespError};
use anyhow::Result;
use tokio_util::codec::{Decoder, Encoder};
use tracing::debug;

#[derive(Debug)]
pub struct RespFrameCodec;

impl RespFrameCodec {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for RespFrameCodec {
    fn default() -> Self {
        Self::new()
    }
}

impl Encoder<Resp> for RespFrameCodec {
    type Error = anyhow::Error;

    fn encode(&mut self, item: Resp, dst: &mut bytes::BytesMut) -> Result<()> {
        debug!("Encoding RESP frame");
        let encoded = item.encode()?;
        debug!("Encoded frame: {:?}", String::from_utf8_lossy(&encoded));
        dst.extend_from_slice(&encoded);
        Ok(())
    }
}

impl Decoder for RespFrameCodec {
    type Item = Resp;
    type Error = anyhow::Error;

    fn decode(&mut self, src: &mut bytes::BytesMut) -> Result<Option<Resp>> {
        match Resp::decode(src) {
            Ok(frame) => Ok(Some(frame)),
            Err(RespError::NotComplete) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
}
