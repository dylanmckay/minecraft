use game::{client, Client};
use game::client::{State, ProtoGame};
use protocol::packet::SpawnPosition;

pub fn spawn_position(client: &mut Client, packet: &SpawnPosition)
    -> Result<(), client::handle::Error> {
    if let State::ProtoGame(ProtoGame::JoinedGame { ref mut game_information }) = client.state {
        game_information.spawn_position = Some(packet.position.into());
        Ok(())
    } else {
        return Err(client::handle::Error::IncorrectState { expected_state: "joined game" });
    }
}

