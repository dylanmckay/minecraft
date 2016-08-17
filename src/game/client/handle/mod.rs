pub use self::login_success::login_success;
pub use self::join_game::join_game;
pub use self::plugin_message::plugin_message;

pub mod login_success;
pub mod join_game;
pub mod plugin_message;

#[derive(Clone, Debug)]
pub enum Error
{
    IncorrectState { expected_state: &'static str },
}

pub fn packet(client: &mut ::game::Client, packet: &::protocol::Packet)
    -> Result<(), Error> {
    use protocol::Packet;

    match *packet {
        Packet::LoginSuccess(ref packet) => self::login_success(client, packet),
        Packet::JoinGame(ref packet) => self::join_game(client, packet),
        Packet::PluginMessage(ref packet) => self::plugin_message(client, packet),
        _ => panic!(";don't know how to handle this packet yet: {:#?}", packet),
    }
}

