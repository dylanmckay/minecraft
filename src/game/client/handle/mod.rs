pub use self::login_success::login_success;
pub use self::join_game::join_game;

pub mod login_success;
pub mod join_game;

pub fn packet(client: &mut ::game::Client, packet: &::protocol::Packet) {
    use protocol::Packet;

    match *packet {
        Packet::LoginSuccess(ref packet) => self::login_success(client, packet),
        Packet::JoinGame(ref packet) => self::join_game(client, packet),
        _ => panic!("don't know how to handle this packet yet: {:#?}", packet),
    }
}

