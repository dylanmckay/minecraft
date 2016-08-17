use protocol::types::VarInt;

/// A raw packet.
#[derive(Debug)]
pub enum Packet
{
    Uncompressed(UncompressedPacket),
    Compressed(CompressedPacket),
}

/// An uncompressed packet.
#[derive(Debug)]
pub struct UncompressedPacket
{
    /// Length of proceding data.
    pub length: VarInt,
    pub packet_id: VarInt,
    pub payload: Vec<u8>,
}

/// A [possibly] compressed packet.
#[derive(Debug)]
pub struct CompressedPacket
{
    /// Length of proceding data.
    pub packet_length: VarInt,

    /// Length of uncompressed packet ID and data.
    /// If the length is set to `0`, the remaining data is *not* compressed.
    pub data_length: VarInt,

    /// The possibly zlib compressed packet ID and data.
    pub packet_data: Vec<u8>,
}

impl CompressedPacket
{
    pub fn is_compressed(&self) -> bool {
        self.data_length.0 != 0
    }
}

