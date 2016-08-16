use io::packet::types::*;

pub enum Packet
{
    /// Packet ID `0x00`
    Handshake(Handshake),
}

