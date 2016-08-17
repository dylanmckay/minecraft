use protocol::packet::raw;
use protocol::types::{Type, VarInt};

use std::io::{Cursor, Write, Read};
use flate2::read::{ZlibEncoder, ZlibDecoder};
use flate2::Compression;

/// Processed packet data (always uncompressed).
#[derive(Clone,Debug)]
pub struct Data
{
    pub packet_id: VarInt,
    pub payload: Vec<u8>,
}

impl Data
{
    /// Reads data from a raw packet.
    pub fn from_raw(packet: raw::Packet) -> Self {
        match packet {
            raw::Packet::Uncompressed(p) => {
                Data {
                    packet_id: p.packet_id,
                    payload: p.payload,
                }
            },
            raw::Packet::Compressed(p) => {
                if p.is_compressed() {
                    // Construct the original compressed payload so we can decode it.
                    let compressed_payload = p.packet_data;

                    let mut decoder = ZlibDecoder::new(Cursor::new(compressed_payload));

                    let mut decompressed_packet_data = Vec::new();
                    decoder.read_to_end(&mut decompressed_packet_data).unwrap();

                    assert_eq!(decompressed_packet_data.len(), p.data_length.0 as _);

                    let mut decompressed_packet_data = Cursor::new(decompressed_packet_data);
                    let packet_id = VarInt::read(&mut decompressed_packet_data).unwrap();
                    let payload = decompressed_packet_data.bytes().map(|a| a.unwrap()).collect();

                    Data {
                        packet_id: packet_id,
                        payload: payload,
                    }
                } else {
                    let mut packet_data = Cursor::new(p.packet_data);
                    let packet_id = VarInt::read(&mut packet_data).unwrap();
                    let payload: Vec<u8> = packet_data.bytes().map(|a| a.unwrap()).collect();

                    Data {
                        packet_id: packet_id,
                        payload: payload,
                    }
                }
            },
        }
    }

    pub fn into_raw(self, compression_threshold: Option<usize>) -> raw::Packet {
        let uncompressed_length = self.packet_id.size_bytes() + self.payload.len();

        if let Some(threshold) = compression_threshold {
            // We only compress the packet if it's big enough.
            let (data_length, possibly_compressed_data): (usize, Vec<u8>) = if uncompressed_length >= threshold {
                let mut uncompressed_packet = Cursor::new(Vec::new());

                self.packet_id.write(&mut uncompressed_packet).unwrap();
                uncompressed_packet.write(&self.payload).unwrap();
                uncompressed_packet.set_position(0);

                let unencryped_length = uncompressed_packet.get_ref().len();

                let encryped_bytes = ZlibEncoder::new(uncompressed_packet, Compression::Default)
                    .bytes().map(|b|b.unwrap()).collect();

                (unencryped_length, encryped_bytes)
            } else {
                let mut packet_data = Cursor::new(Vec::new());

                self.packet_id.write(&mut packet_data).unwrap();
                packet_data.write(&self.payload).unwrap();

                (0, packet_data.into_inner())
            };

            let packet_length = VarInt(data_length as _).size_bytes() + possibly_compressed_data.len();

            raw::Packet::Compressed(raw::CompressedPacket {
                packet_length: VarInt(packet_length as _),
                data_length: VarInt(data_length as _),
                packet_data: possibly_compressed_data,
            })
        } else {

            raw::Packet::Uncompressed(raw::UncompressedPacket {
                length: VarInt(uncompressed_length as _),
                packet_id: self.packet_id,
                payload: self.payload,
            })
        }
    }
}

