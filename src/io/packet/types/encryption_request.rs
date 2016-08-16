use io::types::*;
use io::{packet, Error, Buffer};
use std;

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

    fn parse(data: Vec<u8>) -> Result<Self, Error> {
        let mut cursor = std::io::Cursor::new(data);

        Ok(EncryptionRequest {
            server_id: String::read(&mut cursor)?,
            public_key: ByteArray::read(&mut cursor)?,
            verify_token: ByteArray::read(&mut cursor)?,
        })
    }

    fn write_payload(&self, buffer: &mut Buffer) -> Result<(), Error> {
        self.server_id.write(buffer)?;
        self.public_key.write(buffer)?;
        self.verify_token.write(buffer)?;

        Ok(())
    }
}

