use game::State;

const INITIAL_STATE: State = State::Handshake;

pub struct Client
{
    pub current_state: State,
}

impl Client
{
    pub fn new() -> Self {
        Client {
            current_state: INITIAL_STATE,
        }
    }
}

