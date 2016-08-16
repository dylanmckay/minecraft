use io::packet;
use io::packet::types::*;
use io::Error;
use game;

pub enum Packet
{
    /// Packet ID `0x00`
    Handshake(Handshake),
}

impl Packet
{
    pub fn parse(current_state: game::State, data: packet::Data)
        -> Result<Self, Error> {
        match current_state {
            game::State::Handshake => self::parse::handshake_state(data),
            game::State::Status => unimplemented!(),
            game::State::Login => unimplemented!(),
            game::State::Play => unimplemented!(),
        }
    }
}

mod parse {
    use io::packet::{self, types, Realization};
    use super::Packet;
    use io::Error;

    pub fn handshake_state(data: packet::Data) -> Result<Packet, Error> {
        match data.packet_id {
            types::handshake::PACKET_ID => {
                Ok(Packet::Handshake(types::Handshake::parse(data.data)?))
            },
            _ => unimplemented!(),
        }
    }
}

