use protocol;

/// A game which hasn't quite begun.
pub enum ProtoGame
{
    /// A handshake needs to be sent to the
    /// server before progressing.
    HandshakePending,

    /// A handshake has been sent, and we need to send
    /// a 'login start' packet to continue.
    PendingLoginStart,

    /// A 'login start' has been sent, and we're waiting
    /// on a response from the server.
    PendingLoginResponse,

    /// We have successfully logged in, and are waiting to
    /// receive a 'join game' packet.
    PendingJoin {
        user_information: UserInformation,
    },

    /// We have received a 'join game' packet.
    JoinedGame {
        user_information: UserInformation,
        player_information: PlayerInformation,
    },
}

pub struct UserInformation
{
    pub uuid: String,
    pub username: String,
}

pub struct PlayerInformation
{
    pub entity_id: i32,
}

impl ProtoGame
{
    pub fn new() -> Self { ProtoGame::HandshakePending }

    pub fn protocol_state(&self) -> protocol::GameState {
        match *self {
            ProtoGame::HandshakePending => protocol::GameState::Handshake,
            ProtoGame::PendingLoginStart => protocol::GameState::Login,
            ProtoGame::PendingLoginResponse => protocol::GameState::Login,
            ProtoGame::PendingJoin { .. } => protocol::GameState::Play,
            ProtoGame::JoinedGame { .. } => protocol::GameState::Play,
        }
    }
}

