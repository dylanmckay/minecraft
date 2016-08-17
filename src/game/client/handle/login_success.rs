use game::Client;
use io::packet::LoginSuccess;
use io;

pub fn login_success(client: &mut Client, _packet: &LoginSuccess) {
    client.current_state = io::GameState::Play;

    // FIXME: set username/uuid
}

