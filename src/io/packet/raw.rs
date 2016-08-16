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
    pub length: i32,
    pub packet_id: i32,
    pub data: Vec<u8>,
}

/// A compressed packet.
pub struct CompressedPacket
{
    /// Length of proceding data.
    pub packet_length: i32,

    /// Length of uncompressed packet ID and data (or 0).
    pub data_length: i32,

    /// zlib compressed packet ID.
    pub compressed_packet_id: i32,
    /// zlib compressed packet data.
    pub compressed_data: Vec<u8>,
}

