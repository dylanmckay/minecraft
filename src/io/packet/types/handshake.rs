use io::types::*;
use io::{packet, Error};
use std::io::{Read, Write};

pub const STATE_STATUS: VarInt = VarInt(1);
pub const STATE_LOGIN: VarInt = VarInt(2);

#[derive(Clone, Debug)]
pub struct Handshake
{
    pub protocol_version: VarInt,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: VarInt,
}

impl packet::Realization for Handshake
{
    const PACKET_ID: VarInt = VarInt(0x00);

    fn parse(read: &mut Read) -> Result<Self, Error> {
        Ok(Handshake {
            protocol_version: VarInt::read(read)?,
            server_address: String::read(read)?,
            server_port: u16::read(read)?,
            next_state: VarInt::read(read)?,
        })
    }

    fn write_payload(&self, write: &mut Write) -> Result<(), Error> {
        self.protocol_version.write(write)?;
        self.server_address.write(write)?;
        self.server_port.write(write)?;
        self.next_state.write(write)?;

        Ok(())
    }
}

