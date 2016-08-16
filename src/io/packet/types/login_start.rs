use io::types::*;
use io::{packet, Error};
use std::io::{Read, Write};

#[derive(Clone, Debug)]
pub struct LoginStart
{
    pub username: String,
}

impl packet::Realization for LoginStart
{
    const PACKET_ID: VarInt = VarInt(0x00);
    const DESCRIPTION: &'static str = "login start";

    fn parse(read: &mut Read) -> Result<Self, Error> {
        Ok(LoginStart {
            username: String::read(read)?,
        })
    }

    fn write_payload(&self, write: &mut Write) -> Result<(), Error> {
        self.username.write(write)?;

        Ok(())
    }
}

