use io::types::VarInt;

/// A raw packet.
pub enum Packet
{
    Uncompressed(UncompressedPacket),
    Compressed(CompressedPacket),
}

/// An uncompressed packet.
pub struct UncompressedPacket
{
    /// Length of proceding data.
    pub length: VarInt,
    pub packet_id: VarInt,
    pub data: Vec<u8>,
}

/// A compressed packet.
pub struct CompressedPacket
{
    /// Length of proceding data.
    pub packet_length: VarInt,

    /// Length of uncompressed packet ID and data (or 0).
    pub data_length: VarInt,

    /// zlib compressed packet ID.
    pub compressed_packet_id: VarInt,
    /// zlib compressed packet data.
    pub compressed_data: Vec<u8>,
}

