pub use self::login_success::login_success;

pub mod login_success;

pub fn packet(client: &mut ::game::Client, packet: ::io::Packet) {
    use io::Packet;

    match packet {
        Packet::LoginSuccess(ref packet) => self::login_success(client, packet),
        _ => panic!("unexpected packet"),
    }
}

