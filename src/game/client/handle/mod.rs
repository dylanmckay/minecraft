pub use self::login_success::login_success;
pub use self::join_game::join_game;
pub use self::plugin_message::plugin_message;
pub use self::server_difficulty::server_difficulty;
pub use self::spawn_position::spawn_position;
pub use self::player_abilities::player_abilities;
pub use self::entity_status::entity_status;
pub use self::held_item_change::held_item_change;
pub use self::statistics::statistics;
pub use self::player_list_item::player_list_item;
pub use self::chunk_data::chunk_data;

pub mod login_success;
pub mod join_game;
pub mod plugin_message;
pub mod server_difficulty;
pub mod spawn_position;
pub mod player_abilities;
pub mod entity_status;
pub mod held_item_change;
pub mod statistics;
pub mod player_list_item;
pub mod chunk_data;

#[derive(Clone, Debug)]
pub enum Error
{
    IncorrectState { expected_state: &'static str },
    EnumDecodingError(::game::enums::DecodingError),
    UuidParseError(::uuid::ParseError),
}

pub fn packet(client: &mut ::game::Client, packet: &::protocol::Packet)
    -> Result<(), Error> {
    use protocol::Packet;

    match *packet {
        Packet::Statistics(ref packet) => self::statistics(client, packet),
        Packet::LoginSuccess(ref packet) => self::login_success(client, packet),
        Packet::JoinGame(ref packet) => self::join_game(client, packet),
        Packet::PluginMessage(ref packet) => self::plugin_message(client, packet),
        Packet::ServerDifficulty(ref packet) => self::server_difficulty(client, packet),
        Packet::SpawnPosition(ref packet) => self::spawn_position(client, packet),
        Packet::PlayerAbilities(ref packet) => self::player_abilities(client, packet),
        Packet::EntityStatus(ref packet) => self::entity_status(client, packet),
        Packet::HeldItemChange(ref packet) => self::held_item_change(client, packet),
        Packet::PlayerListItem(ref packet) => self::player_list_item(client, packet),
        Packet::ChunkData(ref packet) => self::chunk_data(client, packet),
        _ => panic!("don't know how to handle this packet yet: {:#?}", packet),
    }
}

impl From<::game::enums::DecodingError> for Error
{
    fn from(e: ::game::enums::DecodingError) -> Self {
        Error::EnumDecodingError(e)
    }
}

impl From<::uuid::ParseError> for Error
{
    fn from(e: ::uuid::ParseError) -> Self {
        Error::UuidParseError(e)
    }
}

