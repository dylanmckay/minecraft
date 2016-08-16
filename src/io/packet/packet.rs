use io::packet::{self, Source};
use io::packet::types::*;
use io::Error;
use game;

#[derive(Clone, Debug)]
pub enum Packet
{
    Handshake(Handshake),
    LoginStart(LoginStart),
    LoginSuccess(LoginSuccess),
    EncryptionRequest(EncryptionRequest),
    EncryptionResponse(EncryptionResponse),
    SetCompression(SetCompression),
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
    use std::io::Cursor;
    use super::Packet;
    use io::Error;

    pub fn handshake_state(source: Source, data: packet::Data) -> Result<Packet, Error> {
        let mut packet_data = Cursor::new(data.data.clone());

        match (source, data.packet_id) {
            (Source::Client, types::Handshake::PACKET_ID) => {
                Ok(Packet::Handshake(types::Handshake::parse(&mut packet_data)?))
            },
            _ => Err(Error::UnknownPacket(data)),
        }
    }

    pub fn login_state(source: Source, data: packet::Data) -> Result<Packet, Error> {
        let mut packet_data = Cursor::new(data.data.clone());

        match (source, data.packet_id) {
            (Source::Client, types::LoginStart::PACKET_ID) => {
                Ok(Packet::LoginStart(types::LoginStart::parse(&mut packet_data)?))
            },
            (Source::Server, types::EncryptionRequest::PACKET_ID) => {
                Ok(Packet::EncryptionRequest(types::EncryptionRequest::parse(&mut packet_data)?))
            },
            (Source::Client, types::EncryptionResponse::PACKET_ID) => {
                Ok(Packet::EncryptionResponse(types::EncryptionResponse::parse(&mut packet_data)?))
            },
            (Source::Server, types::LoginSuccess::PACKET_ID) => {
                Ok(Packet::LoginSuccess(types::LoginSuccess::parse(&mut packet_data)?))
            },
            (Source::Server, types::SetCompression::PACKET_ID) => {
                Ok(Packet::SetCompression(types::SetCompression::parse(&mut packet_data)?))
            },
            _ => Err(Error::UnknownPacket(data)),
        }
    }
}

