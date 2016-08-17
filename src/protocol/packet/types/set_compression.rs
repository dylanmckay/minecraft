use protocol::types::*;
use protocol::{packet, Error};
use std::io::{Read, Write};

#[derive(Clone, Debug)]
pub struct SetCompression
{
    pub threshold: VarInt,
}

impl packet::Realization for SetCompression
{
    const PACKET_ID: VarInt = VarInt(0x03);
    const DESCRIPTION: &'static str = "set compression";

    fn parse(read: &mut Read) -> Result<Self, Error> {
        Ok(SetCompression {
            threshold: VarInt::read(read)?,
        })
    }

    fn write_payload(&self, write: &mut Write) -> Result<(), Error> {
        self.threshold.write(write)?;

        Ok(())
    }
}

