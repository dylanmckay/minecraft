use io::types::*;
use io::{packet, Error, Buffer};
use std;

#[derive(Clone, Debug)]
pub struct SetCompression
{
    pub threshold: VarInt,
}

impl packet::Realization for SetCompression
{
    const PACKET_ID: VarInt = VarInt(0x03);

    fn parse(data: Vec<u8>) -> Result<Self, Error> {
        let mut cursor = std::io::Cursor::new(data);

        Ok(SetCompression {
            threshold: VarInt::read(&mut cursor)?,
        })
    }

    fn write_payload(&self, buffer: &mut Buffer) -> Result<(), Error> {
        self.threshold.write(buffer)?;

        Ok(())
    }
}

