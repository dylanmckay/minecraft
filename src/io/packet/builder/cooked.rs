use io::{Packet, Error};
use io::packet::{self, builder};
use io::types::*;
use game;

use std::io::{Read, Cursor};

/// A cooked packet builder has all batteries included.
pub struct Cooked
{
    raw_builder: builder::Raw,

    /// If packets are bigger than this, they will be compressed.
    compression_threshold: Option<usize>,
}

impl Cooked
{
    pub fn new() -> Self {
        Cooked {
            raw_builder: builder::Raw::new(),
            compression_threshold: None,
        }
    }

    /// Give bytes to the packet builder.
    pub fn give_bytes(&mut self, data: &[u8]) {
        self.raw_builder.give_bytes(data);
    }

    /// Takes a packet off of the queue.
    pub fn take_packet(&mut self,
                       source: packet::Source,
                       game_state: game::State) -> Option<Result<Packet, Error>> {
        if let Some(raw_packet) = self.raw_builder.take_packet() {
            let packet = match self.make_packet(source, game_state, raw_packet) {
                Ok(p) => p,
                Err(e) => return Some(Err(e)),
            };

            self.preprocess_packet(&packet);
            Some(Ok(packet))
        } else {
            None
        }
    }

    pub fn is_compression_enabled(&self) -> bool { self.compression_threshold.is_some() }

    /// Preprocess a packet before it gets lost. Some packets
    /// change the way we parse packets (i.e. setting compression).
    fn preprocess_packet(&mut self, packet: &Packet) {
        match *packet {
            Packet::SetCompression(ref packet) => {
                self.compression_threshold = if packet.threshold.0 >= 0 {
                    Some(packet.threshold.0 as _)
                } else {
                    None
                };
            },
            _ => {
                // We don't care about any other packets.
            }
        }
    }

    fn make_packet(&mut self,
                   source: packet::Source,
                   game_state: game::State,
                   packet: builder::raw::PartialPacket)
        -> Result<Packet, Error> {
        let data = packet::Data::from_raw(self.make_raw_packet(packet));
        Packet::parse(source, game_state, data)
    }

    fn make_raw_packet(&mut self, packet: builder::raw::PartialPacket)
        -> packet::raw::Packet {
        let mut cursor = Cursor::new(packet.payload);

        if self.is_compression_enabled() {
            let data_length = VarInt::read(&mut cursor).unwrap();
            let packet_data: Vec<u8> = cursor.bytes().map(|a| a.unwrap()).collect();

            packet::raw::Packet::Compressed(packet::raw::CompressedPacket {
                packet_length: VarInt(packet.size as _),
                data_length: data_length,
                packet_data: packet_data,
            })
        } else {
            let packet_id = VarInt::read(&mut cursor).unwrap();
            let packet_payload: Vec<u8> = cursor.bytes().map(|a| a.unwrap()).collect();

            packet::raw::Packet::Uncompressed(packet::raw::UncompressedPacket {
                length: VarInt(packet.size as _),
                packet_id: packet_id,
                payload: packet_payload,
            })
        }
    }
}

