use io::types::*;
use io::{packet, Error};
use std::io::{Read, Write};

#[derive(Clone, Debug)]
pub struct LoginSuccess
{
    pub uuid: String,
    pub username: String,
}

impl packet::Realization for LoginSuccess
{
    const PACKET_ID: VarInt = VarInt(0x02);
    const DESCRIPTION: &'static str = "login success";

    fn parse(read: &mut Read) -> Result<Self, Error> {
        Ok(LoginSuccess {
            uuid: String::read(read)?,
            username: String::read(read)?,
        })
    }

    fn write_payload(&self, write: &mut Write) -> Result<(), Error> {
        self.uuid.write(write)?;
        self.username.write(write)?;

        Ok(())
    }
}

