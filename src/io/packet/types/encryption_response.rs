use io::types::*;
use io::{packet, Error};
use std::io::{Read, Write};

#[derive(Clone, Debug)]
pub struct EncryptionResponse
{
    pub shared_secret: ByteArray,
    pub verify_token: ByteArray,
}

impl packet::Realization for EncryptionResponse
{
    const PACKET_ID: VarInt = VarInt(0x01);

    fn parse(read: &mut Read)-> Result<Self, Error> {
        Ok(EncryptionResponse {
            shared_secret: ByteArray::read(read)?,
            verify_token: ByteArray::read(read)?,
        })
    }

    fn write_payload(&self, write: &mut Write) -> Result<(), Error> {
        self.shared_secret.write(write)?;
        self.verify_token.write(write)?;

        Ok(())
    }
}

