use io::types::*;

pub const STATE_STATUS: VarInt = VarInt(1);
pub const STATE_LOGIN: VarInt = VarInt(2);

pub struct Handshake
{
    pub protocol_version: VarInt,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: VarInt,
}

