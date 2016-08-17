use game;
use protocol;
use na;
use uuid;

/// A game which hasn't quite begun.
#[derive(Clone,Debug)]
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
        game_information: GameInformation,
    },
}

#[derive(Clone,Debug)]
pub struct UserInformation
{
    pub uuid: uuid::Uuid,
    pub username: String,
}

#[derive(Clone,Debug)]
pub struct PlayerInformation
{
    pub entity_id: i32,
}

#[derive(Clone,Debug)]
pub struct GameInformation
{
    pub user_information: UserInformation,
    pub player_information: PlayerInformation,

    pub difficulty: Option<game::Difficulty>,
    pub spawn_position: Option<na::Vector3<f32>>,
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

impl GameInformation
{
    pub fn is_complete(&self) -> bool {
        self.difficulty.is_some()
    }
}

