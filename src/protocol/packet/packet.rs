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

    JoinGame(JoinGame),

    EncryptionRequest(EncryptionRequest),
    EncryptionResponse(EncryptionResponse),

    SetCompression(SetCompression),

    PluginMessage(PluginMessage),
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
            JoinGame,
            EncryptionRequest,
            EncryptionResponse,
            SetCompression,
            PluginMessage
        )
    }
}

mod parse {
    use protocol::packet::{self, types, Realization, Source};
    use std::io::Cursor;
    use super::Packet;
    use protocol::Error;

    macro_rules! handle_packets {
        ( $s:expr, $data:expr; $( $source:ident => $ty:ident, )+ ) => {
            {
                let mut packet_payload = Cursor::new($data.payload.clone());

                match ($s, $data.packet_id) {
                    $( ( Source::$source, types::$ty::PACKET_ID) => {
                        Ok(Packet::$ty(types::$ty::parse(&mut packet_payload)?))
                    }, )*
                    _ => Err(Error::UnknownPacket($data)),
                }
            }
        }
    }

    pub fn handshake_state(source: Source, data: packet::Data) -> Result<Packet, Error> {
        handle_packets!(source, data;
            Client => Handshake,
        )
    }

    pub fn login_state(source: Source, data: packet::Data) -> Result<Packet, Error> {
        handle_packets!(source, data;
            Client => LoginStart,
            Server => LoginSuccess,
            Server => SetCompression,
            Server => EncryptionRequest,
            Client => EncryptionResponse,
        )
    }

    pub fn play_state(source: Source, data: packet::Data) -> Result<Packet, Error> {
        handle_packets!(source, data;
            Server => JoinGame,
            Server => PluginMessage,
        )
    }
}

