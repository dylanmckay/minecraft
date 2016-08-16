use io::types::*;
use io::{packet, Error, Buffer};
use std;

#[derive(Clone, Debug)]
pub struct EncryptionResponse
{
    pub shared_secret: ByteArray,
    pub verify_token: ByteArray,
}

impl packet::Realization for EncryptionResponse
{
    const PACKET_ID: VarInt = VarInt(0x01);

    fn parse(data: Vec<u8>) -> Result<Self, Error> {
        let mut cursor = std::io::Cursor::new(data);

        Ok(EncryptionResponse {
            shared_secret: ByteArray::read(&mut cursor)?,
            verify_token: ByteArray::read(&mut cursor)?,
        })
    }

    fn write_payload(&self, buffer: &mut Buffer) -> Result<(), Error> {
        self.shared_secret.write(buffer)?;
        self.verify_token.write(buffer)?;

        Ok(())
    }
}

