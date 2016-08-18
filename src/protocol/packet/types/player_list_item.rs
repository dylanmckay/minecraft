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
    AddPlayer(AddPlayer),
    UpdateGameMode(UpdateGameMode),
    UpdateLatency(UpdateLatency),
    UpdateDisplayName(UpdateDisplayName),
    RemovePlayer,
}

define_composite_type!(AddPlayer => [
    name: String,
    properties: Array<Property>,
    game_mode: VarInt,
    ping: VarInt,
    display_name: Option<Chat>
]);

define_composite_type!(UpdateGameMode => [ game_mode: VarInt ]);
define_composite_type!(UpdateLatency => [ ping: VarInt ]);
define_composite_type!(UpdateDisplayName => [ display_name: Option<Chat> ]);

define_composite_type!(Property => [
    name: String,
    value: String,
    signature: Option<String>
]);

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
    }
}

impl Player
{
    pub fn read(action_id: VarInt, read: &mut Read) -> Result<Self, Error> {
        Ok(Player {
            uuid: Uuid::read(read)?,
            action: Action::read(action_id, read)?,
        })
    }
}

impl Action
{
    pub fn read(action_id: VarInt, read: &mut Read) -> Result<Self, Error> {
        match action_id {
            ADD_PLAYER_ID => Ok(Action::AddPlayer(AddPlayer::read(read)?)),
            UPDATE_GAME_MODE_ID => Ok(Action::UpdateGameMode(UpdateGameMode::read(read)?)),
            UPDATE_LATENCY_ID => Ok(Action::UpdateLatency(UpdateLatency::read(read)?)),
            UPDATE_DISPLAY_NAME_ID => Ok(Action::UpdateDisplayName(UpdateDisplayName::read(read)?)),
            REMOVE_PLAYER_ID => Ok(Action::RemovePlayer),
            d => Err(Error::InvalidDiscriminator("player list item action id", d.0 as _)),
        }
    }
}

