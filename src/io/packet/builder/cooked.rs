use io::{Packet, Error, GameState, connection};
use io::packet::{self, builder};
use io::types::*;

use std::io::{Read, Cursor};

/// A cooked packet builder has all batteries included.
#[derive(Clone, Debug)]
pub struct Cooked
{
    raw_builder: builder::Raw,
}

impl Cooked
{
    pub fn new() -> Self {
        Cooked {
            raw_builder: builder::Raw::new(),
        }
    }

    /// Give bytes to the packet builder.
    pub fn give_bytes(&mut self, data: &[u8]) {
        self.raw_builder.give_bytes(data);
    }

    /// Takes a packet off of the queue.
    pub fn take_packet(&mut self,
                       connection_state: &connection::State,
                       game_state: GameState) -> Option<Result<Packet, Error>> {
        if let Some(raw_packet) = self.raw_builder.take_packet() {
            let packet = match self.make_packet(connection_state, game_state, raw_packet) {
                Ok(p) => p,
                Err(e) => return Some(Err(e)),
            };

            Some(Ok(packet))
        } else {
            None
        }
    }

    fn make_packet(&mut self,
                   connection_state: &connection::State,
                   game_state: GameState,
                   packet: builder::raw::PartialPacket)
        -> Result<Packet, Error> {
        let data = packet::Data::from_raw(self.make_raw_packet(connection_state, packet));
        Packet::parse(connection_state.source, game_state, data)
    }

    fn make_raw_packet(&mut self,
                       connection_state: &connection::State,
                       packet: builder::raw::PartialPacket)
        -> packet::raw::Packet {
        let mut cursor = Cursor::new(packet.payload);

        if connection_state.compression.is_enabled() {
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

