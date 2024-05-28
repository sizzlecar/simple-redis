use crate::RespEncoder;
use crate::{Resp, RespDecoder};
use anyhow::Result;
use tokio_util::codec::{Decoder, Encoder};

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
        let encoded = item.encode()?;
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
            Err(e) => Err(e),
        }
    }
}
