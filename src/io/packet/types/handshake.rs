use io::types::*;
use io::{packet, Error, Buffer};
use std;

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

    fn parse(data: Vec<u8>) -> Result<Self, Error> {
        let mut cursor = std::io::Cursor::new(data);

        Ok(Handshake {
            protocol_version: VarInt::read(&mut cursor)?,
            server_address: String::read(&mut cursor)?,
            server_port: u16::read(&mut cursor)?,
            next_state: VarInt::read(&mut cursor)?,
        })
    }

    fn write_payload(&self, buffer: &mut Buffer) -> Result<(), Error> {
        self.protocol_version.write(buffer)?;
        self.server_address.write(buffer)?;
        self.server_port.write(buffer)?;
        self.next_state.write(buffer)?;

        Ok(())
    }
}

