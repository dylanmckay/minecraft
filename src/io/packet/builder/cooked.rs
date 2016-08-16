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
    _compression_threshold: Option<usize>,
}

impl Cooked
{
    pub fn new() -> Self {
        Cooked {
            raw_builder: builder::Raw::new(),
            _compression_threshold: None,
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

    /// Preprocess a packet before it gets lost. Some packets
    /// change the way we parse packets (i.e. setting compression).
    fn preprocess_packet(&mut self, _packet: &Packet) {
        // Nothing we need to do just yet.
    }

    fn make_packet(&mut self,
                   source: packet::Source,
                   game_state: game::State,
                   packet: builder::raw::PartialPacket)
        -> Result<Packet, Error> {
        let data = packet::Data::new(self.make_raw_packet(packet));
        Packet::parse(source, game_state, data)
    }

    fn make_raw_packet(&mut self, packet: builder::raw::PartialPacket)
        -> packet::raw::Packet {
        let mut cursor = Cursor::new(packet.retrieved_data);

        let packet_id = VarInt::read_from(&mut cursor).unwrap();
        let packet_data: Vec<u8> = cursor.bytes().map(|a| a.unwrap()).collect();

        packet::raw::Packet::Uncompressed(packet::raw::UncompressedPacket {
            length: VarInt(packet.size as _),
            packet_id: packet_id,
            data: packet_data,
        })
    }
}

