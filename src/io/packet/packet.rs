use io::packet::{self, Source};
use io::packet::types::*;
use io::Error;
use game;

pub enum Packet
{
    Handshake(Handshake),
    LoginStart(LoginStart),
    EncryptionRequest(EncryptionRequest),
}

impl Packet
{
    pub fn parse(source: Source, current_state: game::State, data: packet::Data)
        -> Result<Self, Error> {
        match current_state {
            game::State::Handshake => self::parse::handshake_state(source, data),
            game::State::Status => unimplemented!(),
            game::State::Login =>self::parse::login_state(source, data),
            game::State::Play => unimplemented!(),
        }
    }
}

mod parse {
    use io::packet::{self, types, Realization, Source};
    use super::Packet;
    use io::Error;

    pub fn handshake_state(source: Source, data: packet::Data) -> Result<Packet, Error> {
        match (source, data.packet_id) {
            (Source::Client, types::Handshake::PACKET_ID) => {
                Ok(Packet::Handshake(types::Handshake::parse(data.data)?))
            },
            _ => unimplemented!(),
        }
    }

    pub fn login_state(source: Source, data: packet::Data) -> Result<Packet, Error> {
        match (source, data.packet_id) {
            (Source::Client, types::LoginStart::PACKET_ID) => {
                Ok(Packet::LoginStart(types::LoginStart::parse(data.data)?))
            },
            (Source::Server, types::EncryptionRequest::PACKET_ID) => {
                Ok(Packet::EncryptionRequest(types::EncryptionRequest::parse(data.data)?))
            },
            _ => unimplemented!(),
        }
    }
}

