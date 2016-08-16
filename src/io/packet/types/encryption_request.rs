use io::types::*;
use io::{packet, Error};
use std;

#[derive(Clone)]
pub struct EncryptionRequest
{
    pub server_id: String,
    pub public_key: ByteArray,
    pub verify_token: ByteArray,
}

impl packet::Realization for EncryptionRequest
{
    const PACKET_ID: i32 = 0x01;

    fn parse(data: Vec<u8>) -> Result<Self, Error> {
        let mut cursor = std::io::Cursor::new(data);

        Ok(EncryptionRequest {
            server_id: String::read(&mut cursor)?,
            public_key: ByteArray::read(&mut cursor)?,
            verify_token: ByteArray::read(&mut cursor)?,
        })
    }
}

