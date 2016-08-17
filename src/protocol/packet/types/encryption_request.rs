use protocol::types::*;
use protocol::{packet, Error};
use std::io::{Read, Write};

#[derive(Clone, Debug)]
pub struct EncryptionRequest
{
    pub server_id: String,
    pub public_key: ByteArray,
    pub verify_token: ByteArray,
}

impl packet::Realization for EncryptionRequest
{
    const PACKET_ID: VarInt = VarInt(0x01);
    const DESCRIPTION: &'static str = "encryption request";

    fn parse(read: &mut Read) -> Result<Self, Error> {
        Ok(EncryptionRequest {
            server_id: String::read(read)?,
            public_key: ByteArray::read(read)?,
            verify_token: ByteArray::read(read)?,
        })
    }

    fn write_payload(&self, write: &mut Write) -> Result<(), Error> {
        self.server_id.write(write)?;
        self.public_key.write(write)?;
        self.verify_token.write(write)?;

        Ok(())
    }
}

