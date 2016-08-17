use game::Client;
use protocol::packet::JoinGame;

pub fn join_game(client: &mut Client, packet: &JoinGame) {
    // FIXME: implement this
    println!("joining game: {:#?}", packet);
}

