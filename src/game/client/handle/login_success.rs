use game::{Client, State};
use io::packet::LoginSuccess;

pub fn login_success(client: &mut Client, _packet: &LoginSuccess) {
    client.current_state = State::Play;

    // FIXME: set username/uuid
}

