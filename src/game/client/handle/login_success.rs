use game::{client, Client};
use game::client::{State, ProtoGame};
use protocol::packet::LoginSuccess;
use uuid::Uuid;

pub fn login_success(client: &mut Client, packet: &LoginSuccess)
    -> Result<(), client::handle::Error> {
    let user_information = client::proto_game::UserInformation {
        uuid: Uuid::parse_str(&packet.uuid)?,
        username: packet.username.clone(),
    };

    let new_state = if let State::ProtoGame(ProtoGame::PendingLoginResponse) = client.state {
        State::ProtoGame(ProtoGame::PendingJoin {
            user_information: user_information,
        })
    } else {
        return Err(client::handle::Error::IncorrectState { expected_state: "pending login response" });
    };

    client.state = new_state;

    Ok(())
}

