use io::types::*;
use io::{packet, Error, Buffer};
use std;

#[derive(Clone, Debug)]
pub struct LoginSuccess
{
    pub uuid: String,
    pub username: String,
}

impl packet::Realization for LoginSuccess
{
    const PACKET_ID: VarInt = VarInt(0x02);

    fn parse(data: Vec<u8>) -> Result<Self, Error> {
        let mut cursor = std::io::Cursor::new(data);

        Ok(LoginSuccess {
            uuid: String::read(&mut cursor)?,
            username: String::read(&mut cursor)?,
        })
    }

    fn write_payload(&self, buffer: &mut Buffer) -> Result<(), Error> {
        self.uuid.write(buffer)?;
        self.username.write(buffer)?;

        Ok(())
    }
}

