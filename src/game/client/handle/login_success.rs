use game::{client, Client};
use protocol::packet::LoginSuccess;

pub fn login_success(client: &mut Client, packet: &LoginSuccess) {
    let user_information = client::proto_game::UserInformation {
        uuid: packet.uuid.clone(),
        username: packet.username.clone(),
    };


    client.state = client::State::ProtoGame(client::ProtoGame::PendingJoin {
        user_information: user_information,
    });
}

