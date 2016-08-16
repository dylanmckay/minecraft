use io::packet::raw;
use io::types::{Type, VarInt};

use std::io::{Cursor, Write, Read};
use flate2::read::ZlibDecoder;

/// Processed packet data (always uncompressed).
#[derive(Clone,Debug)]
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
            raw::Packet::Compressed(p) => {
                if p.is_compressed() {
                    println!("decompressing, p: {:#?}", p);
                    // Construct the original compressed payload so we can decode it.
                    let compressed_payload = {
                        let mut payload = Cursor::new(Vec::new());

                        p.packet_id.write(&mut payload).unwrap();
                        payload.write(&p.data).unwrap();
                        payload.into_inner()
                    };

                    let mut decoder = ZlibDecoder::new(Cursor::new(compressed_payload));

                    let mut decompressed_payload = Vec::new();
                    decoder.read_to_end(&mut decompressed_payload).unwrap();

                    assert_eq!(decompressed_payload.len(), p.data_length.0 as _);

                    let mut decompressed_payload = Cursor::new(decompressed_payload);
                    let packet_id = VarInt::read(&mut decompressed_payload).unwrap();

                    let mut data = Vec::new();
                    decompressed_payload.read_to_end(&mut data).unwrap();

                    Data {
                        packet_id: packet_id,
                        data: data,
                    }
                } else {
                    Data {
                        packet_id: p.packet_id,
                        data: p.data,
                    }
                }
            },
        }
    }
}

