use protocol::types::*;
use std::io::prelude::*;
use protocol::Error;
use uuid::Uuid;

const ADD_PLAYER_ID: VarInt = VarInt(0);
const UPDATE_GAME_MODE_ID: VarInt = VarInt(1);
const UPDATE_LATENCY_ID: VarInt = VarInt(2);
const UPDATE_DISPLAY_NAME_ID: VarInt = VarInt(3);
const REMOVE_PLAYER_ID: VarInt = VarInt(4);

#[derive(Clone, Debug)]
pub struct PlayerListItem
{
    pub players: Array<Player>,
}

#[derive(Clone, Debug)]
pub struct Player
{
    pub uuid: Uuid,
    pub action: Action,
}

#[derive(Clone, Debug)]
pub enum Action
{
    AddPlayer {
        name: String,
        properties: Array<Property>,
        game_mode: VarInt,
        ping: VarInt,
        display_name: Option<Chat>,
    },
    UpdateGameMode {
        game_mode: VarInt,
    },
    UpdateLatency {
        ping: VarInt,
    },
    UpdateDisplayName {
        display_name: Option<Chat>,
    },
    RemovePlayer,
}

#[derive(Clone, Debug)]
pub struct Property
{
    pub name: String,
    pub value: String,
    pub signature: Option<String>,
}

impl ::protocol::packet::Realization for PlayerListItem
{
    const PACKET_ID: VarInt = VarInt(0x2D);
    const DESCRIPTION: &'static str = "PlayerListItem";

    fn parse(read: &mut Read) -> Result<Self, Error> {
        let action_id = VarInt::read(read)?;
        let players = Array::read_with(read, |read| Player::read(action_id, read))?;

        Ok(PlayerListItem { players: players })
    }

    fn write_payload(&self, _write: &mut Write) -> Result<(), Error> {
        unimplemented!();
        Ok(())
    }
}

impl Player
{
    pub fn read(action_id: VarInt, read: &mut Read) -> Result<Self, Error> {
        match action_id {
            ADD_PLAYER_ID => {
                unimplemented!()
            },
            _ => unimplemented!(),
        }
    }
}

