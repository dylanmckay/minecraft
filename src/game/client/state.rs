use {game, protocol};
use game::client;

pub enum State
{
    Game(game::Context),
    ProtoGame(game::client::ProtoGame),
}

impl State
{
    pub fn initial() -> Self { State::ProtoGame(client::ProtoGame::new()) }

    pub fn protocol_state(&self) -> protocol::GameState {
        match *self {
            State::Game(..) => protocol::GameState::Play,
            State::ProtoGame(ref g) => g.protocol_state(),
        }
    }
}

