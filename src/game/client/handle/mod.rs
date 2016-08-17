pub use self::login_success::login_success;

pub mod login_success;

pub fn packet(client: &mut ::game::Client, packet: &::protocol::Packet) {
    use protocol::Packet;

    match *packet {
        Packet::LoginSuccess(ref packet) => self::login_success(client, packet),
        _ => panic!("don't know how to handle this packet yet: {:#?}", packet),
    }
}

