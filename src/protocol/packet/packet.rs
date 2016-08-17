use protocol::packet::{self, Source, Realization};
use protocol::packet::types::*;
use protocol::{Error, GameState};

use std::io::prelude::*;
use std::io::Cursor;
use std;

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
    pub fn parse(source: Source, current_state: GameState, data: packet::Data)
        -> Result<Self, Error> {
        match current_state {
            GameState::Handshake => self::parse::handshake_state(source, data),
            GameState::Status => unimplemented!(),
            GameState::Login =>self::parse::login_state(source, data),
            GameState::Play => self::parse::play_state(source, data),
        }
    }

    pub fn write(&self, _read: &mut Read) -> Result<(), std::io::Error> {
        unimplemented!();
        // let raw_packet = self.clone().into_data().into_raw();
    }

    pub fn into_data(self) -> packet::Data {
        let mut buffer = Cursor::new(Vec::new());
        macro_rules! handle_packet {
            ( $( $ty:ident),* ) => {
                match self {
                    $(
                        Packet::$ty(packet) => {
                            packet.write_payload(&mut buffer).unwrap();
                            packet::Data {
                                packet_id: packet.packet_id(),
                                payload: buffer.into_inner(),
                            }
                        },
                    )+

                }
            }
        };

        handle_packet!(
            Handshake,
            LoginStart,
            LoginSuccess,
            EncryptionRequest,
            EncryptionResponse,
            SetCompression
        )
    }
}

mod parse {
    use protocol::packet::{self, types, Realization, Source};
    use std::io::Cursor;
    use super::Packet;
    use protocol::Error;

    pub fn handshake_state(source: Source, data: packet::Data) -> Result<Packet, Error> {
        let mut packet_payload = Cursor::new(data.payload.clone());

        match (source, data.packet_id) {
            (Source::Client, types::Handshake::PACKET_ID) => {
                Ok(Packet::Handshake(types::Handshake::parse(&mut packet_payload)?))
            },
            _ => Err(Error::UnknownPacket(data)),
        }
    }

    pub fn login_state(source: Source, data: packet::Data) -> Result<Packet, Error> {
        let mut packet_payload = Cursor::new(data.payload.clone());

        match (source, data.packet_id) {
            (Source::Client, types::LoginStart::PACKET_ID) => {
                Ok(Packet::LoginStart(types::LoginStart::parse(&mut packet_payload)?))
            },
            (Source::Server, types::EncryptionRequest::PACKET_ID) => {
                Ok(Packet::EncryptionRequest(types::EncryptionRequest::parse(&mut packet_payload)?))
            },
            (Source::Client, types::EncryptionResponse::PACKET_ID) => {
                Ok(Packet::EncryptionResponse(types::EncryptionResponse::parse(&mut packet_payload)?))
            },
            (Source::Server, types::LoginSuccess::PACKET_ID) => {
                Ok(Packet::LoginSuccess(types::LoginSuccess::parse(&mut packet_payload)?))
            },
            (Source::Server, types::SetCompression::PACKET_ID) => {
                Ok(Packet::SetCompression(types::SetCompression::parse(&mut packet_payload)?))
            },
            _ => Err(Error::UnknownPacket(data)),
        }
    }

    pub fn play_state(source: Source, data: packet::Data) -> Result<Packet, Error> {
        match (source, data.packet_id) {
            _ => Err(Error::UnknownPacket(data)),
        }
    }
}

