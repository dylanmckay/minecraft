use game::{self, client, Client};
use game::client::{State, ProtoGame};
use protocol::packet::ServerDifficulty;

pub fn server_difficulty(client: &mut Client, packet: &ServerDifficulty)
    -> Result<(), client::handle::Error> {
    if let State::ProtoGame(ProtoGame::JoinedGame { ref mut game_information }) = client.state {
        let difficulty = game::Difficulty::decode(packet.difficulty)?;

        game_information.difficulty = Some(difficulty);
        Ok(())
    } else {
        return Err(client::handle::Error::IncorrectState { expected_state: "joined game" });
    }
}

