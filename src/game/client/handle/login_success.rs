use game::Client;
use protocol::packet::LoginSuccess;
use protocol;

pub fn login_success(client: &mut Client, _packet: &LoginSuccess) {
    client.current_state = protocol::GameState::Play;

    // FIXME: set username/uuid
}

