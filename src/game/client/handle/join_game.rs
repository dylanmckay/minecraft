use game::{client, Client};
use protocol::packet::JoinGame;

pub fn join_game(client: &mut Client, packet: &JoinGame) {
    let new_state = if let client::State::ProtoGame(client::ProtoGame::PendingJoin { ref user_information }) = client.state {
        let player_information = client::proto_game::PlayerInformation {
            entity_id: packet.entity_id,
        };

        client::State::ProtoGame(client::ProtoGame::JoinedGame {
            user_information: user_information.clone(),
            player_information: player_information,
        })
    } else {
        panic!("we shouldn't be getting this.")
    };

    client.state = new_state;
}
