use game::{client, Client};
use protocol::packet::JoinGame;

pub fn join_game(client: &mut Client, packet: &JoinGame)
    -> Result<(), client::handle::Error> {
    let new_state = if let client::State::ProtoGame(client::ProtoGame::PendingJoin { ref user_information }) = client.state {
        let player_information = client::proto_game::PlayerInformation {
            entity_id: packet.entity_id,
        };

        client::State::ProtoGame(client::ProtoGame::JoinedGame {
            game_information: client::proto_game::GameInformation {
                user_information: user_information.clone(),
                player_information: player_information,
                difficulty: None,
                spawn_position: None,
            }
        })
    } else {
        return Err(client::handle::Error::IncorrectState { expected_state: "pending join" });
    };

    client.state = new_state;
    Ok(())
}

