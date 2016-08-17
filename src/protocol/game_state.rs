
#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub enum GameState
{
    Handshake,
    Status,
    Login,
    Play,
}

