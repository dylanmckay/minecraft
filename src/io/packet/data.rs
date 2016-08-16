use io::packet::raw;
use io::types::VarInt;

/// Processed packet data (always uncompressed).
pub struct Data
{
    pub packet_id: VarInt,
    pub data: Vec<u8>,
}

impl Data
{
    /// Reads data from a raw packet.
    pub fn new(packet: raw::Packet) -> Self {
        match packet {
            raw::Packet::Uncompressed(p) => {
                Data {
                    packet_id: p.packet_id,
                    data: p.data,
                }
            },
            raw::Packet::Compressed(..) => {
                unimplemented!();
            },
        }
    }
}

