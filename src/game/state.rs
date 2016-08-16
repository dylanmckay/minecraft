
#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub enum State
{
    Handshake,
    Status,
    Login,
    Play,
}

